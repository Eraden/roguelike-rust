use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::render_on;
use game::sprites::{Animation, Gender};
use game::sprites::RenderPosition;
use game::sprites::Sprite;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;

pub struct WarriorSprite<'a> {
    pub texture: Rc<Texture<'a>>,
    pub frames_per_animation: i32,
    pub tile_size: i32,
    pub source: Rect,
    pub dest: Rect,
    pub animation: Animation,
    pub animation_step: i32,
    pub gender: Gender,
}

impl<'a> WarriorSprite<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let texture = main_renderer
            .texture_manager
            .load("./assets/textures/warrior spritesheet calciumtrice.png")
            .unwrap();

        WarriorSprite {
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
            gender: Gender::Female,
            animation: Animation::Idle,
            animation_step: 0,
        }
    }

    fn resolve_y(&self, animation: &Animation) -> i32 {
        let animation_offset = match animation {
            Animation::Idle => 0,
            Animation::Running => 2,
            Animation::Attacking => 3,
            _ => 0,
        };
        let gender_offset = match self.gender {
            Gender::Male => 0,
            Gender::Female => 5,
        };
        animation_offset + gender_offset
    }

    fn animate(&mut self, ticks: i32, y: i32) {
        let source = &mut self.source;
        let animation_step = (ticks / 300) % self.frames_per_animation;

        self.animation_step = animation_step;

        source.set_x(self.tile_size * animation_step);
        source.set_y(self.tile_size * y);
    }
}

impl<'a> RenderPosition for WarriorSprite<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        render_on(&mut self.dest, &(self.tile_size as usize), x, y);
    }
}

impl<'a> Sprite<'a> for WarriorSprite<'a> {
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
