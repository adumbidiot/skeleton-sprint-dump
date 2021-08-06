mod ui;

use crate::ui::Widget;
use ggez::{
    conf::{
        WindowMode,
        WindowSetup,
    },
    event::{
        self,
        winit_event::{
            Event,
            KeyboardInput,
            WindowEvent,
        },
        EventHandler,
    },
    graphics,
    Context,
    ContextBuilder,
    GameResult,
};
use std::time::Duration;
use stretch::{
    geometry::Size,
    style::{
        Dimension,
        PositionType,
    },
};
use ui::{
    Color,
    RectWidget,
};

struct MainState {
    ui: ui::Manager,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut ui = ui::Manager::new(1920.0, 1080.0);
        let toolbar = RectWidget::new()
            .width(Dimension::Percent(1.0))
            .height(Dimension::Percent(0.1))
            .color(ui::Color::rgb(56, 56, 56));

        let mut main_holder = RectWidget::new()
            .width(Dimension::Percent(1.0))
            .height(Dimension::Percent(0.9))
            .padding(Dimension::Points(20.0));

        let mut block_canvas = RectWidget::new()
            .color(Color::rgb(128, 128, 128))
            .width(Dimension::Percent(0.9))
            .height(Dimension::Percent(1.0));

        let mut block_bar = RectWidget::new()
            .color(Color::rgb(128, 128, 128))
            .height(Dimension::Percent(1.0))
            .width(Dimension::Percent(0.1))
            .margin_left(Dimension::Points(20.0));

        main_holder.add_child(Box::new(block_canvas));
        main_holder.add_child(Box::new(block_bar));

        ui.add_child(Box::new(toolbar));
        ui.add_child(Box::new(main_holder));

        ui.generate_layout();

        Ok(MainState { ui })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        self.ui.render(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let title = "Skeleton Sprint LevelBuilder";

    let (ctx, event_loop) = &mut ContextBuilder::new(&title, "adumbidiot")
        .window_setup(WindowSetup::default().title(&title))
        .window_mode(
            WindowMode::default()
                .dimensions(1920.0, 1080.0)
                .resizable(true),
        )
        .build()
        .expect("Error Initializing Context");

    let mut main = MainState::new(ctx).unwrap();

    //event::run(ctx, event_loop, &mut main);

    let mut rect_widget_2 = RectWidget::new()
        .width(Dimension::Percent(1.0))
        .height(Dimension::Points(100.0))
        .top(Dimension::Points(0.0))
        .color(ui::Color::rgb(128, 128, 128));

    let rect_widget_3 = RectWidget::new()
        .width(Dimension::Points(100.0))
        .height(Dimension::Points(100.0))
        .left(Dimension::Points(0.0))
        .top(Dimension::Points(0.0))
        //.position(PositionType::Absolute)
        .color(ui::Color::rgb(0, 128, 127));

    let rect_widget_4 = RectWidget::new()
        .width(Dimension::Points(100.0))
        .height(Dimension::Points(100.0))
        .position(PositionType::Relative)
        .color(ui::Color::rgb(128, 0, 126));

    rect_widget_2.add_child(Box::new(rect_widget_3));
    rect_widget_2.add_child(Box::new(rect_widget_4));

    /*
    let taskbar = ui::Element::new()
        .position(ui::Position::new(0.0, 0.0, ui::PositionType::Absolute))
        .element_type(ui::ElementType::Rect(1920 / 2, 50))
        .color(ui::Color::new(127, 127, 127, 255));

    let toolbar = ui::Element::new();
        //.position(ui::Position::new(0.0, 0.0, ui::PositionType::Left)

    let taskbar = ui.add_element(taskbar);
    let toolbar = ui.add_element(toolbar);
    */
    while ctx.continuing {
        ctx.timer_context.tick();
        event_loop.poll_events(|event| {
            let event = ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ggez::quit(ctx),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
                        event::KeyCode::Escape => ggez::quit(ctx),
                        _ => (),
                    },
                    WindowEvent::Resized(logical_size) => {
                        let size = logical_size.to_physical(graphics::hidpi_factor(ctx) as f64);
                        dbg!(size.width, size.height);
                        main.ui.resize(size.width as f32, size.height as f32);
                        graphics::set_screen_coordinates(
                            ctx,
                            graphics::Rect::new(0.0, 0.0, size.width as f32, size.height as f32),
                        )
                        .unwrap();
                        main.ui.generate_layout();
                    }
                    _x => {
                        // `CloseRequested` and `KeyboardInput` events won't appear here.
                        //println!("Other window event fired: {:?}", x)
                    }
                },
                _x => {
                    //println!("Device event fired: {:?}", x)
                }
            }
        });

        main.update(ctx).unwrap();
        main.draw(ctx).unwrap();

        ggez::timer::yield_now();
    }
    println!("Hello, world!");
}
