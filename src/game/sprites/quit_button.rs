#![feature(range_contains)]

use game::sprites::Sprite;
use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::Animation;
use sdl2::rect::Rect;
use std::rc::Rc;
use sdl2::render::Texture;
use game::managers::FontDetails;
use game::ui::render_text;

pub struct QuitButtonSprite<'a> {
    pub text_texture: Rc<Texture<'a>>,
    pub text_source: Rect,
    pub text_dest: Rect,
    pub background_texture: Rc<Texture<'a>>,
    pub background_source: Rect,
    pub background_dest: Rect,
}

impl<'a> QuitButtonSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let text_texture: Rc<Texture<'a>> = {
            let font_details = FontDetails::new("./assets/Beyond Wonderland.ttf", 50);
            render_text(&font_details, "Quit", main_renderer)
        };

        let background_texture: Rc<Texture<'a>> = {
            main_renderer.texture_manager
                .load("./assets/button_background.png")
                .unwrap()
        };

        let config = main_renderer.config.clone();
        let top: i32 = (config.height - 200) as i32;
        let background_width: u32 = 300;
        let text_width: u32 = 280;

        QuitButtonSprite {
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
        let dest = &self.background_dest;
        let xs: i32 = dest.x();
        let xe: i32 = xs + dest.width() as i32;
        let ys: i32 = dest.y();
        let ye: i32 = ys + dest.width() as i32;
        (xs..xe).contains(&x) && (ys..ye).contains(&y)
    }
}

impl<'a> Sprite<'a> for QuitButtonSprite<'a> {
    fn update(&mut self, ticks: i32) {}

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
