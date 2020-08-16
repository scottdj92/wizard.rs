use amethyst::{
    ecs::{World, WorldExt},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod wizard;
mod deck;

use crate::wizard::WizardState;
use crate::deck::*;

#[derive(Debug)]
struct PlayerScore {
    pub score: i32,
}

fn main() -> amethyst::Result<()> {
    // set up loggers and load application assets
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    // start renderer
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut world = World::empty();
    // register card entity
    world.register::<deck::Deck>();

    // create scores per player that joined
    let score = PlayerScore {
        score: 0,
    };

    world.insert(score);
    initialize_deck(world);

    let mut game = Application::new(assets_dir, WizardState, game_data)?;
    game.run();

    Ok(())
}

