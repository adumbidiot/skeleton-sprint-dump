use super::{
    Color,
    DrawCall,
};
use stretch::{
    geometry::{
        Rect,
        Size,
    },
    node::Node,
    result::Layout,
    style::{
        Dimension,
        PositionType,
        Style,
    },
};

pub trait Widget {
    fn add_child(&mut self, widget: Box<dyn Widget>);
    fn get_children(&self) -> &[Box<dyn Widget>];
    fn get_node(&mut self) -> &mut Node;
    fn get_color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }

    fn draw(&self, _parent: (f32, f32), _layout: &Layout, _draw_calls: &mut Vec<DrawCall>) {}
}

pub struct RectWidget {
    node: Node,
    color: Color,
    children: Vec<Box<dyn Widget>>,
    test: String,
}

impl RectWidget {
    pub fn new() -> Self {
        RectWidget {
            node: Node::new(Default::default(), Vec::new()),
            color: Default::default(),
            children: Vec::new(),
            test: String::from("Test"),
        }
    }

    pub fn left(mut self, left: Dimension) -> Self {
        let mut style = self.node.style();
        style.position.start = left;
        self.node.set_style(style);
        self
    }

    pub fn right(mut self, right: Dimension) -> Self {
        let mut style = self.node.style();
        style.position.end = right;
        self.node.set_style(style);
        self
    }

    pub fn top(mut self, top: Dimension) -> Self {
        let mut style = self.node.style();
        style.position.top = top;
        self.node.set_style(style);
        self
    }

    pub fn bottom(mut self, bottom: Dimension) -> Self {
        let mut style = self.node.style();
        style.position.bottom = bottom;
        self.node.set_style(style);
        self
    }

    pub fn width(mut self, width: Dimension) -> Self {
        let mut style = self.node.style();
        style.size.width = width;
        self.node.set_style(style);
        self
    }

    pub fn height(mut self, height: Dimension) -> Self {
        let mut style = self.node.style();
        style.size.height = height;
        self.node.set_style(style);
        self
    }

    pub fn position(mut self, position: PositionType) -> Self {
        let mut style = self.node.style();
        style.position_type = position;
        self.node.set_style(style);
        self
    }

    pub fn margin(mut self, margin: Dimension) -> Self {
        let mut style = self.node.style();
        style.margin = Rect {
            start: margin,
            end: margin,
            top: margin,
            bottom: margin,
        };
        self.node.set_style(style);
        self
    }

    pub fn margin_left(mut self, value: Dimension) -> Self {
        let mut style = self.node.style();
        style.margin.start = value;
        self.node.set_style(style);
        self
    }

    pub fn padding(mut self, padding: Dimension) -> Self {
        let mut style = self.node.style();
        style.padding = Rect {
            start: padding,
            end: padding,
            top: padding,
            bottom: padding,
        };
        self.node.set_style(style);
        self
    }

    pub fn size(mut self, size: Size<Dimension>) -> Self {
        let mut style = self.node.style();
        style.size = size;
        self.node.set_style(style);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.node.set_style(style);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for RectWidget {
    fn add_child(&mut self, mut widget: Box<dyn Widget>) {
        self.node.add_child(widget.get_node());
        self.children.push(widget);
    }

    fn get_children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    fn get_node(&mut self) -> &mut Node {
        &mut self.node
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }

    fn draw(&self, parent: (f32, f32), layout: &Layout, draw_calls: &mut Vec<DrawCall>) {
        let x = parent.0 + layout.location.x;
        let y = parent.1 + layout.location.y;
        draw_calls.push(DrawCall::Rect(
            x,
            y,
            layout.size.width,
            layout.size.height,
            self.color.clone(),
        ));
        draw_calls.push(DrawCall::Text(x, y, "TEST", self.color.clone()));

        for (child, layout) in self.children.iter().zip(layout.children.iter()) {
            child.draw((x, y), layout, draw_calls);
        }
    }
}

pub struct TextWidget {
    node: Node,
    color: Color,
    children: Vec<Box<dyn Widget>>,
    font: String,
}

impl Widget for TextWidget {
    fn add_child(&mut self, mut widget: Box<dyn Widget>) {
        self.node.add_child(widget.get_node());
        self.children.push(widget);
    }

    fn get_children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    fn get_node(&mut self) -> &mut Node {
        &mut self.node
    }

    fn get_color(&self) -> Color {
        self.color.clone()
    }
}
