use amethyst::prelude::*;

struct Main;

impl EmptyState for Main {
    fn on_start(&mut self, _data: StateData<'_, ()>) {
        println!("Hello world");
    }

    fn on_stop(&mut self, _data: StateData<'_, ()>) {
        println!("Goodbye world");
    }

    fn update(&mut self, _data: StateData<'_, ()>) -> EmptyTrans {
        println!("hello from amethyst");
        Trans::Quit
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let mut game = Application::new("/", Main, ())?;
    game.run();

    Ok(())
}
