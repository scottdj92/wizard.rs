use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::winit::*;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Hello world");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Goodbye world");
    }

    // fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    //     println!("hello from amethyst");
    //     Trans::Quit
    // }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                Event::WindowEvent { event , .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None
                },
                _ => Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    // set up loggers and load application assets
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // start renderer
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
            .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(app_root, GameState, game_data)?;
    game.run();

    Ok(())
}
