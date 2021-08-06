mod widget;

pub use self::widget::{
    RectWidget,
    Widget,
};
use ggez::{
    graphics::{
        self,
        DrawParam,
        Text,
    },
    nalgebra,
    Context,
};
use std::{
    collections::HashMap,
    time::Instant,
};
use stretch::{
    geometry::Size,
    node::Node,
    number::Number,
    result::Layout,
    style::{
        AlignContent,
        AlignItems,
        Dimension,
        FlexWrap,
        JustifyContent,
        Style,
    },
};

#[derive(Debug)]
pub enum DrawCall<'a> {
    Rect(f32, f32, f32, f32, Color),
    Text(f32, f32, &'a str, Color),
}

#[derive(Debug, Default, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Into<ggez::graphics::Color> for Color {
    fn into(self) -> ggez::graphics::Color {
        ggez::graphics::Color::from_rgba(self.r, self.g, self.b, self.a)
    }
}

impl Into<ggez::graphics::Color> for &Color {
    fn into(self) -> ggez::graphics::Color {
        ggez::graphics::Color::from_rgba(self.r, self.g, self.b, self.a)
    }
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
}

fn dimension_as_f32(dimension: Dimension) -> Option<f32> {
    match dimension {
        Dimension::Points(n) => Some(n),
        _ => None,
    }
}

pub struct Manager {
    node: Node,
    pub children: Vec<Box<dyn Widget>>,
    layout: Layout,
    //draw_calls: Vec<DrawCall>,
    text_cache: HashMap<String, Text>,
}

impl Manager {
    pub fn new(width: f32, height: f32) -> Self {
        let node = Node::new(
            Style {
                size: Size {
                    width: Dimension::Points(width),
                    height: Dimension::Points(height),
                },
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::FlexStart,
                ..Default::default()
            },
            Vec::new(),
        );
        let layout = node
            .compute_layout(Size {
                width: Number::Defined(width),
                height: Number::Defined(height),
            })
            .unwrap();
        Self {
            node,
            children: Vec::new(),
            layout,
            text_cache: HashMap::new(),
            //draw_calls: Vec::new(),
        }
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        let mut style = self.node.style();
        style.size.width = Dimension::Points(w);
        style.size.height = Dimension::Points(h);
        self.node.set_style(style);
    }

    pub fn generate_layout(&mut self) {
        let size = self.node.style().size;
        let w = dimension_as_f32(size.width).unwrap_or(0.0);
        let h = dimension_as_f32(size.height).unwrap_or(0.0);

        let start = Instant::now();
        self.layout = self
            .node
            .compute_layout(Size {
                width: Number::Defined(w),
                height: Number::Defined(h),
            })
            .unwrap();
        let end = Instant::now();
        dbg!(end - start);
        //dbg!(&self.layout);
    }

    pub fn add_child(&mut self, mut widget: Box<dyn Widget>) {
        self.node.add_child(widget.get_node());
        self.children.push(widget);
    }

    pub fn render(&mut self, ctx: &mut Context) {
        //let draw_calls = &mut self.draw_calls;
        let mut draw_calls = Vec::new();
        self.layout
            .children
            .iter()
            .zip(self.children.iter())
            .map(DisplayData::from)
            .for_each(|display_data| {
                display_data
                    .widget
                    .draw((0.0, 0.0), &display_data.layout, &mut draw_calls);
            });

        dbg!(&draw_calls);
        /*
        self.layout
            .children
            .iter()
            .zip(self.children.iter())
            .map(DisplayData::from)
            .for_each(|display_data| render_widget(ctx, (0.0, 0.0), &display_data));*/

        for draw_call in draw_calls.iter() {
            match draw_call {
                DrawCall::Rect(x, y, w, h, color) => {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(*x, *y, *w, *h),
                        color.into(),
                    )
                    .unwrap();
                    graphics::draw(ctx, &rect, DrawParam::default()).unwrap();
                }
                DrawCall::Text(x, y, val, color) => {
                    let text = self //TODO: Make not stupid
                        .text_cache
                        .entry(val.to_string())
                        .or_insert(Text::new(*val));
                    graphics::draw(
                        ctx,
                        text,
                        DrawParam::default().dest(nalgebra::Point2::new(*x, *y)),
                    )
                    .unwrap();
                    ggez::graphics::pop_transform(ctx);
                    ggez::graphics::apply_transformations(ctx);
                }
                _ => {}
            }
        }

        draw_calls.clear();
    }
}

struct DisplayData<'a> {
    widget: &'a dyn Widget,
    layout: &'a Layout,
}

impl<'a> From<(&'a dyn Widget, &'a Layout)> for DisplayData<'a> {
    fn from((widget, layout): (&'a dyn Widget, &'a Layout)) -> Self {
        DisplayData { widget, layout }
    }
}

impl<'a> From<(&'a Layout, &'a dyn Widget)> for DisplayData<'a> {
    fn from((layout, widget): (&'a Layout, &'a dyn Widget)) -> Self {
        DisplayData { widget, layout }
    }
}

impl<'a> From<(&'a Layout, &'a Box<dyn Widget>)> for DisplayData<'a> {
    fn from((layout, widget): (&'a Layout, &'a Box<dyn Widget>)) -> Self {
        DisplayData {
            widget: &**widget,
            layout,
        }
    }
}
/*
fn render_widget(ctx: &mut Context, parent_position: (f32, f32), display_data: &DisplayData) {
    let x = display_data.layout.location.x + parent_position.0;
    let y = display_data.layout.location.y + parent_position.1;
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            0.0,
            0.0,
            display_data.layout.size.width,
            display_data.layout.size.height,
        ),
        display_data.widget.get_color().into(),
    )
    .unwrap();

    graphics::draw(ctx, &rect, (nalgebra::Point2::new(x, y),)).unwrap();

    display_data
        .layout
        .children
        .iter()
        .zip(display_data.widget.get_children().iter())
        .map(DisplayData::from)
        .for_each(|child_data| render_widget(ctx, (x, y), &child_data));
}
*/
