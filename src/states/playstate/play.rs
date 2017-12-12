use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::ecs::World;
use amethyst::renderer::{PosTex, Camera, Mesh, Material, PngFormat, Projection};
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::assets::Loader;
use amethyst::core::cgmath::{Vector3, Deg};

use amethyst::prelude::*;

use super::tilemap;

pub struct PlayState;

impl State for PlayState {
    fn on_start(&mut self, world: &mut World) {
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

    let (mesh, material) = {
        let loader = world.read_resource::<Loader>();

        let mesh: Handle<Mesh> =
        loader.load_from_data(tilemap::gen_tilemap_plane().into(), (), &world.read_resource());

        let mat_defaults = world.read_resource::<MaterialDefaults>();

        let tex_storage = world.read_resource();

        let tilemap_material = Material {
            albedo: loader.load(
                "../resources/scifitiles-sheet_0.png",
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
    transform.translation = Vector3::new(192.0, 128.0, 0.0);
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .with(Transform::default())
        .build();
}
