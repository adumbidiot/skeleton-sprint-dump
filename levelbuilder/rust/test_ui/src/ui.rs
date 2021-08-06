pub use ggez::graphics::Color;
use ggez::GameResult;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Handle(u64);

impl Handle {
    pub const MAIN: Self = Handle(0);
}

/// A handle that locks a ui instance
pub struct HandleLock<'a> {
    handle: Handle,
    ui: &'a mut Ui,
}

impl HandleLock<'_> {
    pub fn add_child<T: Element + 'static>(&mut self, el: Box<T>) {
        let handle = self.ui.allocate_el(self.handle, el);
        self.ui
            .elements
            .get_mut(&self.handle)
            .unwrap()
            .children
            .push(handle);
    }
}

pub struct ElStorage {
    element: Box<dyn Element>,
	
    children: Vec<Handle>,
    parent: Option<Handle>,
	
	region: Option<Handle>,
}

pub struct Ui {
    elements: HashMap<Handle, ElStorage>,
    last_handle: u64,

    should_draw: bool,
}

impl Ui {
    pub fn new(width: usize, height: usize) -> Self {
        let mut elements: HashMap<Handle, _> = HashMap::new();
        let main = Rect::new(Layout {
            x: Unit::Points(0),
            y: Unit::Points(0),
            width: Unit::Points(width),
            height: Unit::Points(height),
        });
        elements.insert(
            Handle::MAIN,
            ElStorage {
                element: Box::new(main),
                children: Vec::new(),
                parent: None,
				region: None,
            },
        );

        Ui {
            elements,
            last_handle: 0,
            should_draw: true,
        }
    }

    pub fn main(&mut self) -> HandleLock {
        HandleLock {
            handle: Handle::MAIN,
            ui: self,
        }
    }

    pub fn should_draw(&self) -> bool {
        self.should_draw
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        let store = self.elements.get(&Handle::MAIN).unwrap();
        let el = &store.element;
        let children = &store.children;

        let region = Region {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };
        el.draw(ctx, &region)?;

        for handle in children {
            let store = self.elements.get(&handle).unwrap();

            store.element.draw(
                ctx,
                &Region {
                    x: 100,
                    y: 100,
                    width: 100,
                    height: 100,
                },
            )?;
        }

        self.should_draw = false;
        Ok(())
    }

    /// Get a new inner handle
    fn generate_handle(&mut self) -> Handle {
        self.last_handle += 1;
        Handle(self.last_handle)
    }

    /// Allocate el and return an inner handle to it
    fn allocate_el<T: Element + 'static>(
        &mut self,
        parent_handle: Handle,
        element: Box<T>,
    ) -> Handle {
        let handle = self.generate_handle();
        let storage = ElStorage {
            element,
            children: Vec::new(),
            parent: Some(parent_handle),
			region: None,
        };
        self.elements.insert(handle, storage);
        handle
    }
	
	/// Uni directional (Hopefully)
	fn calc_region_from(&mut self, handle: Handle) {
		let el = self.elements.get(handle).unwrap();
		// TODO: Region Calc
	}
}

pub trait Element {
    fn draw(&self, _ctx: &mut ggez::Context, _region: &Region) -> GameResult;
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub x: Unit,
    pub y: Unit,
    pub width: Unit,
    pub height: Unit,
}

pub struct Region {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct Rect {
    layout: Layout,
    color: Color,
}

impl Rect {
    pub fn new(layout: Layout) -> Self {
        Rect {
            layout,
            color: Color::from_rgba(255, 255, 0, 255),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Element for Rect {
    fn draw(&self, ctx: &mut ggez::Context, region: &Region) -> GameResult {
        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            [
                region.x as f32,
                region.y as f32,
                region.width as f32,
                region.height as f32,
            ]
            .into(),
            self.color,
        )?;
        ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Unit {
    Points(usize),
    Percent(usize),
}
