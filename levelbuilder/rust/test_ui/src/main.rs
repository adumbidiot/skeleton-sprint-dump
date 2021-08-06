mod ui;

use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("ui-test", "adumbidiot")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Ui Test")
                .vsync(true),
        )
        .window_mode(ggez::conf::WindowMode::default().maximized(false))
        .build()
        .unwrap();

    let mut game = Main::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Main {
    ui: ui::Ui,
    frames: usize,
}

impl Main {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut ui = ui::Ui::new(800, 600);
        let mut main = ui.main();

        let mut child = ui::Rect::new(ui::Layout {
            x: ui::Unit::Points(100),
            y: ui::Unit::Points(100),
            width: ui::Unit::Points(100),
            height: ui::Unit::Points(100),
        });

        child.set_color(ggez::graphics::BLACK);

        main.add_child(child.into());

        Self { ui, frames: 0 }
    }
}

impl EventHandler for Main {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.ui.should_draw() || self.frames == 0 {
            graphics::clear(ctx, graphics::WHITE);
            self.ui.draw(ctx)?;
            graphics::present(ctx)?;
        }

        self.frames += 1;
        self.frames %= 100;
        Ok(())
    }
}
