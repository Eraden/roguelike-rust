use game::sprites::Sprite;
use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use sdl2::rect::Rect;
use std::rc::Rc;
use sdl2::render::Texture;
use game::managers::FontDetails;
use game::ui::render_text;
use game::sprites::check_is_inside;

pub struct StartButtonSprite<'a> {
    pub text_texture: Rc<Texture<'a>>,
    pub text_source: Rect,
    pub text_dest: Rect,
    pub background_texture: Rc<Texture<'a>>,
    pub background_source: Rect,
    pub background_dest: Rect,
}

impl<'a> StartButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let text_texture: Rc<Texture<'a>> = {
            let font_details = FontDetails::new("./assets/Beyond Wonderland.ttf", 50);
            render_text(&font_details, "Start", main_renderer)
        };

        let background_texture: Rc<Texture<'a>> = {
            main_renderer.texture_manager
                .load("./assets/button_background.png")
                .unwrap()
        };

        let config = main_renderer.config.clone();
        let top: i32 = (config.height - 350) as i32;
        let background_width: u32 = 300;
        let text_width: u32 = 280;

        StartButtonSprite {
            text_texture,
            text_source: Rect::new(0, 0, text_width, 50),
            text_dest: Rect::new(
                ((config.width / 2) - (background_width / 2)) as i32,
                top - 5,
                text_width,
                90
            ),
            background_texture,
            background_source: Rect::new(0, 0, 765, 296),
            background_dest: Rect::new(
                ((config.width / 2) - (background_width / 2)) as i32,
                top,
                background_width,
                100
            ),
        }
    }

    pub fn is_inside(&self, x: &i32, y: &i32) -> bool {
        check_is_inside(x, y, &self.background_dest)
    }
}

impl<'a> Sprite<'a> for StartButtonSprite<'a> {
    fn update(&mut self, _ticks: i32) {}

    fn render(&self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>) {
        canvas.copy_ex(
            &self.background_texture,
            Some(self.background_source), Some(self.background_dest),
            0.0, None,
            false, false,
        ).unwrap();

        canvas.copy_ex(
            &self.text_texture,
            Some(self.text_source), Some(self.text_dest),
            0.0, None,
            false, false,
        ).unwrap();
    }
}
