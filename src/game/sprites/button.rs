use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::managers::FontDetails;
use game::sprites::*;
use game::ui::render_text;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;
use game::config::Config;

const TEXT_MARGIN: u32 = 40;

pub struct ButtonSprite<'a> {
    text_texture: Rc<Texture<'a>>,
    text_source: Rect,
    text_dest: Rect,
    background_texture: Rc<Texture<'a>>,
    background_source: Rect,
    background_dest: Rect,
    text_width: u32,
}

impl<'a> ButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>, text: &str, left: i32, top: i32) -> Self {
        let font_details = FontDetails::new("./assets/fonts/Beyond Wonderland.ttf", 90);
        let (text_width, text_height) = main_renderer
            .font_manager
            .load(&font_details)
            .unwrap()
            .size_of(text)
            .unwrap();
        let text_texture: Rc<Texture<'a>> = render_text(&font_details, text, main_renderer);

        let background_texture: Rc<Texture<'a>> = main_renderer
            .texture_manager
            .load("./assets/textures/button_background.png")
            .unwrap();

        let background_width: u32 = text_width + (TEXT_MARGIN * 2);

        let text_dest = Rect::new(
            left + TEXT_MARGIN as i32,
            top - 5,
            text_width,
            text_height,
        );
        let background_dest = Rect::new(
            left,
            top,
            background_width,
            100,
        );

        Self {
            text_texture,
            text_source: Rect::new(0, 0, text_width, text_height),
            text_dest,
            background_texture,
            background_source: Rect::new(0, 0, 765, 296),
            background_dest,
            text_width,
        }
    }

    pub fn is_inside(&self, x: &i32, y: &i32) -> bool {
        check_is_inside(x, y, &self.background_dest)
    }

    pub fn center_x(&mut self, config: &Config) {
        let text_width = self.text_width.clone();
        let background_width: u32 = text_width + (TEXT_MARGIN * 2);
        self.text_dest.set_x(((config.width / 2) - (text_width / 2)) as i32);
        self.background_dest.set_x(((config.width / 2) - (background_width / 2)) as i32);
    }
}

//impl<'a> RenderPosition for ButtonSprite<'a> {
//    /** Move sprite by it's texture size */
//    fn render_on(&mut self, x: &usize, y: &usize) {}
//
//    /** Move by pixels */
//    fn move_by(&mut self, x: i32, y: i32) {}
//
//    /** Move to coords in pixels */
//    fn move_to(&mut self, x: i32, y: i32) {}
//}

impl<'a> Sprite<'a> for ButtonSprite<'a> {
    fn update(&mut self, _ticks: i32) {}

    fn render(&mut self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>) {
        canvas
            .copy_ex(
                &self.background_texture,
                Some(self.background_source),
                Some(self.background_dest),
                0.0,
                None,
                false,
                false,
            ).unwrap();

        canvas
            .copy_ex(
                &self.text_texture,
                Some(self.text_source),
                Some(self.text_dest),
                0.0,
                None,
                false,
                false,
            ).unwrap();
    }
}
