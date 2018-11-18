use game::app::WindowCanvas;
use game::config::Config;
use game::main_renderer::MainRenderer;
use game::managers::FontDetails;
use game::sprites::*;
use game::ui::render_text;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;

const TEXT_MARGIN: u32 = 40;

pub struct ButtonSprite<'a> {
    text_texture: Rc<Texture<'a>>,
    text_source: Rect,
    text_dest: Rect,
    background_texture: Rc<Texture<'a>>,
    background_source: Rect,
    background_dest: Rect,
}

impl<'a> ButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>, text: &str, left: i32, top: i32) -> Self {
        let font_details = FontDetails::new("./assets/fonts/Beyond Wonderland.ttf", 90);
        let generated = render_text(&font_details, text, main_renderer);

        let background_texture: Rc<Texture<'a>> = main_renderer
            .texture_manager
            .load("./assets/textures/button_background.png")
            .unwrap();

        let background_width: u32 = generated.width + (TEXT_MARGIN * 2);

        let text_dest = Rect::new(left + TEXT_MARGIN as i32, top - 5, generated.width, generated.height);
        let background_dest = Rect::new(left, top, background_width, 100);

        Self {
            text_texture: generated.tex,
            text_source: Rect::new(0, 0, generated.width, generated.height),
            text_dest,
            background_texture,
            background_source: Rect::new(0, 0, 765, 296),
            background_dest,
        }
    }

    pub fn is_inside(&self, x: &i32, y: &i32) -> bool {
        check_is_inside(x, y, &self.background_dest)
    }

    pub fn center_x(&mut self, config: &Config) {
        let text_width = self.text_source.width();
        let background_width: u32 = text_width + (TEXT_MARGIN * 2);
        self.text_dest
            .set_x(((config.width / 2) - (text_width / 2)) as i32);
        self.background_dest
            .set_x(((config.width / 2) - (background_width / 2)) as i32);
    }
}

impl<'a> Sprite<'a> for ButtonSprite<'a> {
    fn update(&mut self, _ticks: i32) {}

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        main_renderer.render_texture(canvas, &self.background_texture, self.background_source, self.background_dest);
        main_renderer.render_texture(canvas, &self.text_texture, self.text_source, self.text_dest);
    }
}
