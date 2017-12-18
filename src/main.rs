extern crate amethyst;
extern crate genmesh;
extern crate tiled;
extern crate gfx;
#[macro_use]
extern crate derivative;

use amethyst::renderer::{Stage, Pipeline, PosTex, RenderBundle, RenderSystem, DisplayConfig};
use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;

mod states;

use states::playstate::play::PlayState;
use states::playstate::tilemap_pass::DrawTilemap;

const BACKGROUND_COLOUR: [f32; 4] = [0.0, 0.0, 0.0, 0.0]; // black

fn run() -> Result<(), amethyst::Error> {
    let display_config_path = format!(
        "{}/config/display.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let resources = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));
    let config = DisplayConfig::load(&display_config_path);

    let mut game = Application::build(resources, PlayState)?
        .with_bundle(RenderBundle::new())?
        .with_bundle(TransformBundle::new())?;
    let pipe = {

        Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target(BACKGROUND_COLOUR, 1.0)
                .with_pass(DrawTilemap::<PosTex>::new())
        )
    };
    game = game.with_local(RenderSystem::build(pipe, Some(config))?);
    Ok(game.build()?.run())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}
