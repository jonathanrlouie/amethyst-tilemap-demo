use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::renderer::PosTex;
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};
use tiled;

pub fn gen_tilemap_plane() -> Vec<PosTex> {
    let tilesize = 32;
    let tilemap_width = 12;
    let tilemap_height = 8;
    let plane = Plane::subdivide(tilemap_width as usize, tilemap_height as usize);

    let half_width = (tilesize * tilemap_width) / 2;
    let half_height = (tilesize * tilemap_height) / 2;

    let vertex_data: Vec<PosTex> = plane.shared_vertex_iter().map(|(raw_x, raw_y)| {
        let vertex_x = half_width as f32 * raw_x;
        let vertex_y = half_height as f32 * raw_y;

        let u_pos = (1.0 + raw_x) / 2.0;
        let v_pos = (1.0 + raw_y) / 2.0;
        let tilemap_x = (u_pos * tilemap_width as f32).floor();
        let tilemap_y = (v_pos * tilemap_height as f32).floor();

        PosTex {
            position: [vertex_x, vertex_y, 0.0],
            tex_coord: [tilemap_x as f32, tilemap_y as f32]
        }
    }).collect();

    let indexed_vertex_data: Vec<PosTex> = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| *vertex_data.get(i as usize).unwrap_or(&PosTex{position: [0., 0., 0.], tex_coord: [0., 0.]}))
        .collect();

    indexed_vertex_data
}

/*
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TilemapImage {
    pub source: String
}

impl Component for TilemapImage {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TilemapDimensions {
    pub map_width: u32,
    pub map_height: u32,
    pub tile_size: u32
}

impl Component for TilemapDimensions {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TilemapTiles {
    pub tiles: Vec<[f32; 4]>,
}

impl Component for TilemapTiles {
    type Storage = DenseVecStorage<Self>;
}*/