use game::config::Config;
use game::managers::FontManager;
use game::managers::TextureManager;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct MainRenderer<'a, 'b> {
    pub config: Config,
    pub font_manager: FontManager<'a>,
    pub texture_manager: TextureManager<'b, WindowContext>,
    pub texture_creator: &'b TextureCreator<WindowContext>,
}

impl<'a, 'b> MainRenderer<'a, 'b> {
    pub fn new(config: Config, font_context: &'a Sdl2TtfContext, texture_creator: &'b TextureCreator<WindowContext>) -> Self {
        MainRenderer {
            config,
            font_manager: FontManager::new(&font_context),
            texture_manager: TextureManager::new(&texture_creator),
            texture_creator,
        }
    }
}
