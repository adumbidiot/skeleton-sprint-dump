use ggez::Context;
use ggez::GameResult;
use std::collections::HashMap;
pub use stretch::geometry::Rect;
use stretch::geometry::Size;
use stretch::node::Node;
pub use stretch::style::Dimension;
use stretch::style::Style;

pub struct SuiContext {
    stretch: stretch::node::Stretch,
    elements: HashMap<ElementHandle, Element>,
}

impl SuiContext {
    pub fn new() -> Self {
        let stretch = stretch::node::Stretch::new();
        SuiContext {
            stretch,
            elements: HashMap::new(),
        }
    }

    pub fn get_el(&self, h: ElementHandle) -> Option<&Element> {
        self.elements.get(&h)
    }

    pub fn get_el_mut(&mut self, h: ElementHandle) -> Option<&mut Element> {
        self.elements.get_mut(&h)
    }

    pub fn add_child(&mut self, parent: ElementHandle, child: ElementHandle) {
        let parent = self.elements.get_mut(&parent).unwrap();
        self.stretch.add_child(parent.node, child.node).unwrap();
    }
}

pub struct Sui {
    ctx: SuiContext,
    main: ElementHandle,
}

impl Sui {
    pub fn new() -> Self {
        let width = 800;
        let height = 600;
        let mut ctx = SuiContext::new();
        let main = ElementBuilder::new()
            .width(Dimension::Points(800.0))
            .height(Dimension::Points(600.0))
            .build(&mut ctx);

        Sui { ctx, main }
    }

    pub fn get_ctx_mut(&mut self) -> &mut SuiContext {
        &mut self.ctx
    }

    pub fn get_main(&mut self) -> ElementHandle {
        self.main
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ctx
            .stretch
            .compute_layout(self.main.node, Size::undefined())
            .unwrap();

        let parent_layout = self.ctx.stretch.layout(self.main.node).unwrap().clone();
        let children = self.ctx.stretch.children(self.main.node).unwrap();

        for node in children.iter() {
            let handle = ElementHandle { node: *node };
            handle.draw(ctx, &mut self.ctx, &parent_layout);
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementHandle {
    node: Node,
}

impl ElementHandle {
    fn compute_layout<'a>(&'a self, ctx: &'a mut SuiContext) -> &'a stretch::result::Layout {
        ctx.stretch.layout(self.node).unwrap()
    }
	
	fn draw(&self, ctx: &mut Context, ui_ctx: &mut SuiContext, parent_layout: &stretch::result::Layout) -> GameResult<()> {
		use ggez::graphics::Color;
        use ggez::graphics::DrawMode;
		
		let child_layout = self.compute_layout(ui_ctx).clone();
		let el = ui_ctx.get_el(*self).unwrap();
		
		 match el.kind {
            ElementKind::Rect => {
				dbg!(&parent_layout, &child_layout);
                let rect = ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::Fill(Default::default()),
                    [0.0, 0.0, child_layout.size.width, child_layout.size.height].into(),
                    Color::new(1.0, 0.0, 0.0, 1.0),
                )?;
                ggez::graphics::draw(
                    ctx,
                    &rect,
                    ggez::graphics::DrawParam::default().dest([
                        child_layout.location.x + parent_layout.location.x,
                        child_layout.location.y + parent_layout.location.y,
                    ]),
                )?;
            }
            ElementKind::None => {}
            _ => unimplemented!(),
        }
		
		let children = ui_ctx.stretch.children(self.node).unwrap();
        for node in children.iter() {
            let handle = ElementHandle { node: *node };
            handle.draw(ctx, ui_ctx, &child_layout);
        }
		
		Ok(())
	}
}

pub struct Element {
    node: stretch::node::Node,
    kind: ElementKind,
}

impl Element {
    fn get_handle(&self) -> ElementHandle {
        ElementHandle { node: self.node }
    }

    fn compute_layout<'a>(&'a self, ctx: &'a mut SuiContext) -> &'a stretch::result::Layout {
        ctx.stretch.compute_layout(self.node, Size::undefined());
        ctx.stretch.layout(self.node).unwrap()
    }
}

#[derive(Debug, Default)]
pub struct ElementBuilder {
    size: Size<Dimension>,
	position: Rect<Dimension>,
	
    kind: ElementKind,
	//color: 
}

impl ElementBuilder {
    pub fn new() -> ElementBuilder {
        Default::default()
    }

    pub fn width(mut self, d: Dimension) -> Self {
        self.size.width = d;
        self
    }

    pub fn height(mut self, d: Dimension) -> Self {
        self.size.height = d;
        self
    }
	
	pub fn left(mut self, d: Dimension) -> Self {
		self.position.start = d;
		self
	}

    pub fn kind(mut self, kind: ElementKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn build(self, ctx: &mut SuiContext) -> ElementHandle {
        let node = {
            ctx.stretch
                .new_node(
                    Style {
                        size: self.size,
						position: self.position,
                        ..Default::default()
                    },
                    Vec::new(),
                )
                .unwrap()
        };

        let el = Element {
            node,
            kind: self.kind,
        };
        let handle = el.get_handle();
        ctx.elements.insert(handle, el);
        handle
    }
}

#[derive(Debug)]
pub enum ElementKind {
    None,
    Rect,
}

impl Default for ElementKind {
    fn default() -> Self {
        Self::None
    }
}
