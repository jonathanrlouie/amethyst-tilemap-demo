use std::marker::PhantomData;

use amethyst::assets::AssetStorage;
use amethyst::core::cgmath::{Matrix4, One, SquareMatrix};
use amethyst::core::transform::Transform;

use amethyst::ecs::{Fetch, Join, ReadStorage};

use amethyst::renderer::{ActiveCamera, Camera, Material, MaterialDefaults, Encoder, Factory, Texture,
                         Position, Query, TexCoord, Mesh, MeshHandle};
use amethyst::renderer::error::Result;

use amethyst::renderer::pipe::pass::{Pass, PassData};
use amethyst::renderer::pipe::{DepthMode, Effect, NewEffect};

use gfx::pso::buffer::ElemStride;

use super::tilemap::{TilemapDimensions, TilesheetDimensions, TilemapTiles};

const TILEMAP_VERT_SRC: &[u8] = include_bytes!("../../../resources/tilemap_v.glsl");
const TILEMAP_FRAG_SRC: &[u8] = include_bytes!("../../../resources/tilemap_f.glsl");


#[derive(Clone, Copy, Debug)]
struct VertexArgs {
    proj: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct FragmentArgs {
    u_world_size: [f32; 4],
    u_tilesheet_size: [f32; 4]
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TileMapBuffer {
    u_data: [[f32; 4]; 4096]
}


/// Draw mesh without lighting
/// `V` is `VertexFormat`
#[derive(Derivative, Clone, Debug, PartialEq)]
#[derivative(Default(bound = "V: Query<(Position, TexCoord)>, Self: Pass"))]
pub struct DrawTilemap<V> {
    _pd: PhantomData<V>,
}

impl<V> DrawTilemap<V>
where
    V: Query<(Position, TexCoord)>,
    Self: Pass,
{
    /// Create instance of `DrawFlat` pass
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a, V> PassData<'a> for DrawTilemap<V>
where
    V: Query<(Position, TexCoord)>,
{
    type Data = (
        Option<Fetch<'a, ActiveCamera>>,
        ReadStorage<'a, Camera>,
        Fetch<'a, AssetStorage<Mesh>>,
        Fetch<'a, AssetStorage<Texture>>,
        Fetch<'a, MaterialDefaults>,
        ReadStorage<'a, MeshHandle>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, TilemapDimensions>,
        ReadStorage<'a, TilesheetDimensions>,
        ReadStorage<'a, TilemapTiles>,
    );
}

impl<V> Pass for DrawTilemap<V>
where
    V: Query<(Position, TexCoord)>,
{
    fn compile(&self, effect: NewEffect) -> Result<Effect> {
        use std::mem;
        effect
            .simple(TILEMAP_VERT_SRC, TILEMAP_FRAG_SRC)
            .with_raw_constant_buffer("VertexArgs", mem::size_of::<VertexArgs>(), 1)
            .with_raw_vertex_buffer(V::QUERIED_ATTRIBUTES, V::size() as ElemStride, 0)
            .with_raw_constant_buffer("TileMapBuffer", mem::size_of::<TileMapBuffer>(), 1)
            .with_raw_constant_buffer("FragmentArgs", mem::size_of::<FragmentArgs>(), 1)
            .with_texture("TilesheetTexture")
            .with_output("Color", Some(DepthMode::LessEqualWrite))
            .build()
    }

    fn apply<'a, 'b: 'a>(
        &'a mut self,
        encoder: &mut Encoder,
        effect: &mut Effect,
        _factory: Factory,
        (active, camera, mesh_storage, tex_storage, material_defaults, mesh, material, global, tilemap_dimensions, tilesheet_dimensions, tile_data): (
            Option<Fetch<'a, ActiveCamera>>,
            ReadStorage<'a, Camera>,
            Fetch<'a, AssetStorage<Mesh>>,
            Fetch<'a, AssetStorage<Texture>>,
            Fetch<'a, MaterialDefaults>,
            ReadStorage<'b, MeshHandle>,
            ReadStorage<'b, Material>,
            ReadStorage<'b, Transform>,
            ReadStorage<'b, TilemapDimensions>,
            ReadStorage<'b, TilesheetDimensions>,
            ReadStorage<'b, TilemapTiles>,
        ),
    ) {
        let camera: Option<(&Camera, &Transform)> = active
            .and_then(|a| {
                let cam = camera.get(a.entity);
                let transform = global.get(a.entity);
                cam.into_iter().zip(transform.into_iter()).next()
            })
            .or_else(|| (&camera, &global).join().next());

        let mesh_storage = &mesh_storage;
        let tex_storage = &tex_storage;
        let material_defaults = &material_defaults;

        for (mesh, material, global, tilemap_dimensions, tilesheet_dimensions, tile_data) in (&mesh, &material, &global, &tilemap_dimensions, &tilesheet_dimensions, &tile_data).join() {
            let mesh = match mesh_storage.get(mesh) {
                Some(mesh) => mesh,
                None => continue,
            };
            let vbuf = match mesh.buffer(V::QUERIED_ATTRIBUTES) {
                Some(vbuf) => vbuf.clone(),
                None => return,
            };

            let vertex_args = camera
                .as_ref()
                .map(|&(ref cam, ref transform)| {
                    VertexArgs {
                        proj: cam.proj.into(),
                        view: transform.0.invert().unwrap().into(),
                        model: *global.as_ref(),
                    }
                })
                .unwrap_or(
                    VertexArgs {
                        proj: Matrix4::one().into(),
                        view: Matrix4::one().into(),
                        model: *global.as_ref(),
                    }
                );

            let tilesheet_texture = tex_storage
                .get(&material.albedo)
                .or_else(|| tex_storage.get(&material_defaults.0.albedo))
                .unwrap();

            effect.update_constant_buffer("VertexArgs", &vertex_args, encoder);
            effect.data.textures.push(tilesheet_texture.view().clone());
            effect.data.samplers.push(tilesheet_texture.sampler().clone());

            let fragment_args = FragmentArgs {
                u_world_size: [tilemap_dimensions.width as f32, tilemap_dimensions.height as f32, 0.0, 0.0],
                u_tilesheet_size: [tilesheet_dimensions.width as f32, tilesheet_dimensions.height as f32, 0.0, 0.0]
            };
            effect.update_buffer("TileMapBuffer", &tile_data.tiles[..], encoder);
            effect.update_constant_buffer("FragmentArgs", &fragment_args, encoder);

            effect.data.vertex_bufs.push(vbuf);

            effect.draw(mesh.slice(), encoder);
            effect.clear();
        }
    }
}
