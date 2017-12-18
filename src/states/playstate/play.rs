use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::ecs::World;
use amethyst::renderer::{Camera, Mesh, PngFormat, Projection};
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::assets::Loader;
use amethyst::core::cgmath::Vector3;

use amethyst::prelude::*;

use std::path::Path;
use std::fs::File;
use tiled::parse;

use super::tilemap;

pub struct PlayState;

impl State for PlayState {
    fn on_start(&mut self, world: &mut World) {
        world.register::<tilemap::TilemapDimensions>();
        world.register::<tilemap::TilesheetDimensions>();
        world.register::<tilemap::TilemapTiles>();
        initialise_camera(world);
        initialise_tilemap(world);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}

fn initialise_camera(world: &mut World) {
    use amethyst::core::cgmath::{Matrix4, Vector3};
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            500.0,
            500.0,
            0.0,
        )))
        .with(Transform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build();
}


fn initialise_tilemap(world: &mut World) {

    use amethyst::assets::Handle;
    use amethyst::renderer::{Material, MaterialDefaults};

    let map_file = match File::open(&Path::new("./resources/map.tmx")) {
        Err(e) => {
            eprintln!("Error opening .tmx file: {}", e);
            return
        },
        Ok(f) => f,
    };
    let map = match parse(map_file) {
        Err(e) => {
            eprintln!("Error while parsing .tmx file: {}", e);
            return
        },
        Ok(m) => m,
    };
    let (tileset, tileset_img) = match map.tilesets.get(0) {
        Some(tileset) => match tileset.images.get(0) {
            Some(img) => (tileset, img),
            None => return
        },
        None => return
    };
    let tileset_width = tileset_img.width as u32 / tileset.tile_width;
    let tileset_height = tileset_img.height as u32 / tileset.tile_height;
    let image_source = &tileset_img.source;

    let tilemap_dimensions = tilemap::TilemapDimensions {
        width: map.width,
        height: map.height
    };

    let tilesheet_dimensions = tilemap::TilesheetDimensions {
        width: tileset_width,
        height: tileset_height
    };

    let tiles = tilemap::TilemapTiles {
        tiles: tilemap::generate_tile_data(&map, tileset_width, tileset_height)
    };

    let half_width: f32 = ((map.width * map.tile_width) / 2) as f32;
    let half_height: f32 = ((map.height * map.tile_height) / 2) as f32;

    let (mesh, material) = {
        let loader = world.read_resource::<Loader>();

        let mesh: Handle<Mesh> =
        loader.load_from_data(tilemap::generate_tilemap_plane(map.tile_width, map.width, map.height).into(), (), &world.read_resource());

        let mat_defaults = world.read_resource::<MaterialDefaults>();

        let tex_storage = world.read_resource();

        let tilemap_material = Material {
            albedo: loader.load(
                format!("{}{}", "../resources/", image_source),
                PngFormat,
                Default::default(),
                (),
                &tex_storage,
            ),
            ..mat_defaults.0.clone()
        };

        (mesh, tilemap_material)
    };

    let mut transform = LocalTransform::default();
    transform.translation = Vector3::new(half_width, half_height, 0.0);
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .with(Transform::default())
        .with(tilemap_dimensions)
        .with(tilesheet_dimensions)
        .with(tiles)
        .build();
}
