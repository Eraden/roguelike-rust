use game::config::Config;
use game::managers::FontManager;
use game::managers::TextureManager;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Viewport {
    pub width: i32,
    pub height: i32,
}

pub struct MainRenderer<'a, 'b> {
    pub config: Config,
    pub font_manager: FontManager<'a>,
    pub texture_manager: TextureManager<'b, WindowContext>,
    pub texture_creator: &'b TextureCreator<WindowContext>,
    pub scroll: Position,
    pub viewport: Viewport,
}

impl<'a, 'b> MainRenderer<'a, 'b> {
    pub fn new(
        config: Config,
        font_context: &'a Sdl2TtfContext,
        texture_creator: &'b TextureCreator<WindowContext>,
    ) -> Self {
        Self {
            config,
            font_manager: FontManager::new(&font_context),
            texture_manager: TextureManager::new(&texture_creator),
            texture_creator,
            scroll: Position { x: 0, y: 0 },
            viewport: Viewport {
                width: 40,
                height: 40,
            },
        }
    }
}
