use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::graphics::Image;
use ggez::graphics::Rect;
use ggez::input::mouse::MouseButton;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use sks::block::BackgroundType;
use sks::block::Block;
use sks::render::ImageRenderer;
use sks::render::ImageRequest;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

const FONT_BYTES: &[u8] = include_bytes!("./OpenSans-Regular.ttf");
const BG_COLOR: Color = ggez::graphics::BLACK;
const PRIMARY_COLOR: Color = Color::new(119.0 / 255.0, 119.0 / 255.0, 119.0 / 255.0, 1.0);
const MENU_COLOR: Color = Color::new(56.0 / 255.0, 56.0 / 255.0, 56.0 / 255.0, 1.0);
const ACTIVE_COLOR: Color = Color::new(255.0, 0.0, 0.0, 1.0);

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("levelbuilder", "skeleton-sprint")
        .window_setup(
            WindowSetup::default()
                .title("Skeleton Sprint Levelbuilder")
                .vsync(true),
        )
        .window_mode(WindowMode::default().resizable(true))
        .build()
        .unwrap();

    let mut my_game = Main::new(&mut ctx).unwrap();

    match ggez::event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

//struct EventQueue(VecDeque<GameEvent>);

struct Main {
    layout_manager: LayoutManager,

    menu_bar: MenuBar,
    board: Board,
    toolbar: ToolBar,

    state: GlobalState,
}

impl Main {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let screen_size = ggez::graphics::screen_coordinates(ctx);
        let layout_manager = LayoutManager::new(screen_size.w, screen_size.h);
        let state = GlobalState::new();

        Ok(Main {
            layout_manager: layout_manager.clone(),

            menu_bar: MenuBar::new(layout_manager.clone()),
            board: Board::new(ctx, layout_manager.clone(), state.clone())?,
            toolbar: ToolBar::new(ctx, layout_manager, state.clone())?,

            state,
        })
    }
}

impl EventHandler for Main {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, BG_COLOR);
        self.menu_bar.draw(ctx)?;
        self.board.draw(ctx)?;
        self.toolbar.draw(ctx)?;
        ggez::graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                let toolbar_layout = self.layout_manager.toolbar_layout();
                if toolbar_layout.contains([x, y]) {
                    let target = (y - toolbar_layout.y) as usize / 50;
                    self.toolbar.toggle(target);

                    return;
                }

                let board_bg_layout = self.layout_manager.board_bg_layout();
                if board_bg_layout.contains([x, y]) {
                    if let Some(block) = self.state.get_active_block() {
                        let board_click_x = x - board_bg_layout.x;
                        let board_click_y = y - board_bg_layout.y;
                        let x_index = ((board_click_x / board_bg_layout.w)
                            * sks::LEVEL_WIDTH as f32)
                            as usize;
                        let y_index = ((board_click_y / board_bg_layout.h)
                            * sks::LEVEL_HEIGHT as f32)
                            as usize;

                        let index = y_index * sks::LEVEL_WIDTH + x_index;
                        dbg!(index);
                        self.state.get_level_builder().set_block(index, block);
                    }
                    return;
                }
            }
            _ => {}
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let _ = ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height));
        self.layout_manager.recalculate_layout(width, height);
    }
}

#[derive(Clone)]
struct LayoutManager(Rc<RefCell<LayoutManagerInner>>);

impl LayoutManager {
    pub fn new(width: f32, height: f32) -> Self {
        Self(Rc::new(RefCell::new(LayoutManagerInner::new(
            width, height,
        ))))
    }

    pub fn menu_bar_layout(&self) -> Rect {
        self.0.borrow().menu_bar
    }

    pub fn board_layout(&self) -> Rect {
        self.0.borrow().board
    }

    pub fn toolbar_layout(&self) -> Rect {
        self.0.borrow().toolbar
    }

    pub fn board_bg_layout(&self) -> Rect {
        self.0.borrow().board_bg
    }

    pub fn recalculate_layout(&self, width: f32, height: f32) {
        self.0.borrow_mut().recalculate_layout(width, height);
    }
}

struct LayoutManagerInner {
    width: f32,
    height: f32,

    menu_bar: Rect,
    board: Rect,
    toolbar: Rect,
    board_bg: Rect,
}

impl LayoutManagerInner {
    pub fn new(width: f32, height: f32) -> Self {
        let mut ret = Self {
            width,
            height,
            menu_bar: Rect::zero(),
            board: Rect::zero(),
            toolbar: Rect::zero(),
            board_bg: Rect::zero(),
        };

        ret.recalculate_layout(width, height);
        ret
    }

    pub fn recalculate_layout(&mut self, width: f32, height: f32) {
        let border_size = 10.0;
        let double_border_size = 2.0 * border_size;

        let menu_bar_height = 50.0;

        let toolbar_width = 50.0;

        let board_y = menu_bar_height + border_size;

        self.width = width;
        self.height = height;

        self.menu_bar = Rect::new(0.0, 0.0, self.width, menu_bar_height);
        self.toolbar = Rect::new(
            (self.width - (toolbar_width + border_size)),
            board_y,
            toolbar_width,
            (self.height - menu_bar_height) - double_border_size,
        );

        self.board = Rect::new(
            border_size,
            board_y,
            (self.width - (toolbar_width + double_border_size)) - border_size,
            self.toolbar.h,
        );

        let level_width = sks::LEVEL_WIDTH as f32;
        let level_height = sks::LEVEL_HEIGHT as f32;

        let width_ratio = self.board.w / level_width;
        let height_ratio = self.board.h / level_height;

        let board_bg_ratio = if width_ratio < height_ratio {
            width_ratio
        } else {
            height_ratio
        };

        self.board_bg = Rect::new(
            self.board.x,
            board_y,
            level_width * board_bg_ratio,
            level_height * board_bg_ratio,
        );
    }
}

struct MenuBar {
    layout_manager: LayoutManager,
}

impl MenuBar {
    pub fn new(layout_manager: LayoutManager) -> Self {
        Self { layout_manager }
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> GameResult<()> {
        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            self.layout_manager.menu_bar_layout(),
            MENU_COLOR,
        )?;
        ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;
        Ok(())
    }
}

struct Board {
    layout_manager: LayoutManager,
    bg_store: HashMap<sks::block::BackgroundType, Image>,
    global_state: GlobalState,
}

impl Board {
    pub fn new(
        ctx: &mut ggez::Context,
        layout_manager: LayoutManager,
        global_state: GlobalState,
    ) -> GameResult<Self> {
        use image::GenericImageView;

        let mut bg_store = HashMap::new();
        bg_store.insert(
            sks::block::BackgroundType::Cobble,
            global_state
                .get_rendered_block(ImageRequest {
                    w: 1920,
                    h: 1080,
                    block: sks::block::Block::Background {
                        background_type: sks::block::BackgroundType::Cobble,
                    },
                })
                .map(|i| {
                    Image::from_rgba8(
                        ctx,
                        i.width() as u16,
                        i.height() as u16,
                        &i.clone().into_rgba().into_vec(),
                    )
                })
                .transpose()?
                .unwrap(),
        );
        Ok(Board {
            layout_manager,
            bg_store,
            global_state,
        })
    }

    pub fn handle_click(&self, x: f32, y: f32) {}

    pub fn draw(&self, ctx: &mut ggez::Context) -> GameResult<()> {
        let board = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            self.layout_manager.board_layout(),
            PRIMARY_COLOR,
        )?;

        ggez::graphics::draw(ctx, &board, ggez::graphics::DrawParam::default())?;

        let board_bg_layout = self.layout_manager.board_bg_layout();
        let board_bg = self
            .bg_store
            .get(&sks::block::BackgroundType::Cobble)
            .unwrap();

        let scale = [
            board_bg_layout.w / board_bg.width() as f32,
            board_bg_layout.h / board_bg.height() as f32,
        ];
        ggez::graphics::draw(
            ctx,
            board_bg,
            ggez::graphics::DrawParam::default()
                .dest([board_bg_layout.x, board_bg_layout.y])
                .scale(scale),
        )?;

        let level_builder = self.global_state.get_level_builder();
        let block_filler = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 60.0, 60.0),
            ACTIVE_COLOR,
        )?;
        for i in 0..sks::LEVEL_SIZE {
            if let Some(block) = level_builder.get_block(i) {
                if block != Block::Empty {
                    ggez::graphics::draw(
                        ctx,
                        &block_filler,
                        ggez::graphics::DrawParam::default()
                            .dest([
                                (i as f32 % sks::LEVEL_WIDTH as f32 * 60.0 * scale[0])
                                    + board_bg_layout.x,
                                ((i / sks::LEVEL_WIDTH) as f32 * 60.0 * scale[1])
                                    + board_bg_layout.y,
                            ])
                            .scale(scale),
                    )?;
                }
            }
        }

        Ok(())
    }
}

struct ToolBar {
    layout_manager: LayoutManager,
    block_list: Vec<Block>,
    image_cache: Vec<Option<Image>>,
    global_state: GlobalState,
}

impl ToolBar {
    pub fn new(
        ctx: &mut ggez::Context,
        layout_manager: LayoutManager,
        global_state: GlobalState,
    ) -> GameResult<Self> {
        use image::GenericImageView;

        let block_list = vec![Block::Block, Block::Lock, Block::Key, Block::Scaffold];
        let image_cache: Vec<_> = block_list
            .iter()
            .cloned()
            .map(|block| {
                global_state
                    .get_rendered_block(ImageRequest {
                        w: 50,
                        h: 50,
                        block,
                    })
                    .map(|i| {
                        Image::from_rgba8(
                            ctx,
                            i.width() as u16,
                            i.height() as u16,
                            &i.clone().into_rgba().into_vec(),
                        )
                    })
                    .transpose()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            layout_manager,
            image_cache,
            block_list,
            global_state,
        })
    }

    pub fn toggle(&mut self, index: usize) {
        let block = self.block_list.get(index);

        if let Some(block) = block {
            let current = self.global_state.get_active_block();
            if current.as_ref() == Some(block) {
                self.global_state.set_active_block(None);
            } else {
                self.global_state.set_active_block(Some(block.clone()));
            }
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> GameResult<()> {
        let block_width = 50.0;

        let base_layout = self.layout_manager.toolbar_layout();
        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            base_layout,
            PRIMARY_COLOR,
        )?;
        ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

        for ((i, image), block) in self
            .image_cache
            .iter()
            .flatten()
            .enumerate()
            .zip(self.block_list.iter())
        {
            let block_dest = [base_layout.x, base_layout.y + (i as f32 * block_width)];
            let active_block = self.global_state.get_active_block();

            ggez::graphics::draw(
                ctx,
                image,
                ggez::graphics::DrawParam::default().dest(block_dest),
            )?;

            if Some(block) == active_block.as_ref() {
                let rect = ggez::graphics::Mesh::new_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    [0.0, 0.0, block_width, block_width].into(),
                    Color::new(ACTIVE_COLOR.r, ACTIVE_COLOR.g, ACTIVE_COLOR.b, 0.5),
                )?;
                ggez::graphics::draw(
                    ctx,
                    &rect,
                    ggez::graphics::DrawParam::default().dest(block_dest),
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
struct GlobalState(Rc<GlobalStateInner>);

impl GlobalState {
    pub fn new() -> Self {
        Self(Rc::new(GlobalStateInner::new()))
    }

    pub fn get_active_block(&self) -> Option<Block> {
        self.0.active_block.borrow().clone()
    }

    pub fn set_active_block(&self, active: Option<Block>) {
        *self.0.active_block.borrow_mut() = active;
    }

    pub fn get_rendered_block(&self, r: ImageRequest) -> Option<image::DynamicImage> {
        self.0.image_renderer.borrow_mut().get_rendered(r).cloned()
    }

    pub fn get_background_type(&self) -> sks::block::BackgroundType {
        self.0.background.clone()
    }

    pub fn get_level_builder(&self) -> LevelBuilderHandle {
        self.0.level_builder.clone()
    }
}

struct GlobalStateInner {
    active_block: RefCell<Option<Block>>,
    image_renderer: RefCell<ImageRenderer>,
    background: sks::block::BackgroundType,
    level_builder: LevelBuilderHandle,
}

impl GlobalStateInner {
    pub fn new() -> Self {
        GlobalStateInner {
            active_block: RefCell::new(None),
            image_renderer: RefCell::new(ImageRenderer::new()),
            background: sks::block::BackgroundType::Cobble,
            level_builder: LevelBuilderHandle::new(),
        }
    }
}

#[derive(Clone)]
pub struct LevelBuilderHandle(Rc<RefCell<LevelBuilder>>);

impl LevelBuilderHandle {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(LevelBuilder::new())))
    }

    pub fn get_block(&self, i: usize) -> Option<Block> {
        self.0.borrow().blocks.get(i).cloned()
    }

    pub fn set_block(&self, i: usize, block: Block) {
        if let Some(block_ref) = self.0.borrow_mut().blocks.get_mut(i) {
            *block_ref = block;
        }
    }
}

pub struct LevelBuilder {
    blocks: Vec<Block>,
    bg: BackgroundType,
    is_dark: bool,
}

impl LevelBuilder {
    pub fn new() -> Self {
        LevelBuilder {
            blocks: vec![Block::Empty; sks::LEVEL_SIZE],
            bg: BackgroundType::Cobble,
            is_dark: false,
        }
    }
}

struct LayoutSystem(Rc<RefCell<LayoutSystemInner>>);

impl LayoutSystem {
    pub fn new() -> Self {
        LayoutSystem(Rc::new(RefCell::new(LayoutSystemInner::new())))
    }
	
	pub fn alloc_node(&self) -> Node {
		// self.0
	}
}

struct LayoutSystemInner {
    nodes: Vec<LayoutStorage>,
    id: usize,
	free_ids: Vec<usize>,
}

impl LayoutSystemInner {
    pub fn new() -> Self {
        LayoutSystemInner {
            nodes: Vec::new(),
            id: 0,
			free_ids: Vec::new(),
        }
    }
}

struct LayoutStorage {
    node: LayoutNode,
    ref_count: usize,
}

struct LayoutNode {
    layout: Rect,
}
