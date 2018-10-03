use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::render_on;
use game::sprites::Animation;
use game::sprites::RenderPosition;
use game::sprites::Sprite;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;

pub struct FemaleDeerSprite<'a> {
    pub texture: Rc<Texture<'a>>,
    pub frames_per_animation: i32,
    pub tile_size: i32,
    pub source: Rect,
    pub dest: Rect,
    pub animation: Animation,
    pub animation_step: i32,
}

impl<'a> FemaleDeerSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let texture = main_renderer
            .texture_manager
            .load("./assets/textures/deer female calciumtrice.png")
            .unwrap();

        FemaleDeerSprite {
            texture,
            frames_per_animation: 5,
            tile_size: 32,
            source: Rect::new(0, 0, 32, 32),
            dest: Rect::new(
                0,
                0,
                main_renderer.config.render_tile.width,
                main_renderer.config.render_tile.height,
            ),
            animation: Animation::Eating,
            animation_step: 0,
        }
    }

    fn resolve_y(&self, animation: &Animation) -> i32 {
        match animation {
            Animation::Idle => 0,
            Animation::Eating => 1,
            Animation::Running => 2,
        }
    }

    fn animate(&mut self, ticks: i32, y: i32) {
        let source = &mut self.source;
        let animation_step = (ticks / 300) % self.frames_per_animation;

        self.animation_step = animation_step;

        source.set_x(self.tile_size * animation_step);
        source.set_y(self.tile_size * y);
    }
}

impl<'a> RenderPosition for FemaleDeerSprite<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        render_on(&mut self.dest, &(self.tile_size as usize), x, y);
    }
}

impl<'a> Sprite<'a> for FemaleDeerSprite<'a> {
    fn update(&mut self, ticks: i32) {
        let y = {
            let animation = &self.animation;
            self.resolve_y(animation)
        };
        self.animate(ticks, y);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>) {
        canvas
            .copy_ex(
                &self.texture,
                Some(self.source),
                Some(self.dest),
                0.0,
                None,
                false,
                false,
            ).unwrap();
    }
}
