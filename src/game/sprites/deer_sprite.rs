use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::*;
use sdl2::rect::Rect;

const TILE_SIZE: u32 = 32;

pub struct DeerSprite<'a> {
    animatable: Animatable,
    renderable: Renderable<'a>,
}

impl<'a> DeerSprite<'a> {
    fn new(main_renderer: &mut MainRenderer<'a, 'a>, path: &str) -> Self {
        let config = { main_renderer.config.clone() };
        let renderable = Renderable::new(
            main_renderer,
            path,
            32,
            Rect::new(0, 0, TILE_SIZE, TILE_SIZE),
            Rect::new(
                0,
                0,
                config.render_tile.width,
                config.render_tile.height,
            ),
        );
        let mut animatable = Animatable::new(5, 32);
        animatable.animation = Animation::Eating;

        Self { renderable, animatable }
    }

    pub fn new_female(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        Self::new(main_renderer, "./assets/textures/deer female calciumtrice.png")
    }

    pub fn new_male(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        Self::new(main_renderer, "./assets/textures/deer male calciumtrice.png")
    }

    pub fn resize(&mut self, size: &u32) {
        self.renderable.dest_size = size.clone() as i32;
        self.renderable.dest = Rect::new(0, 0, size.clone(), size.clone());
    }

    fn resolve_y(&self, animation: &Animation) -> i32 {
        match animation {
            Animation::Idle => 0,
            Animation::Eating => 1,
            Animation::Running => 2,
            _ => 0,
        }
    }
}

impl<'a> RenderPosition for DeerSprite<'a> {
    fn render_on(&mut self, x: &usize, y: &usize) {
        let tile_size: i32 = self.renderable.dest_size();
        render_on(&mut self.renderable.dest, &(tile_size as usize), x, y);
    }

    fn move_by(&mut self, x: i32, y: i32) {
        let dest = &mut self.renderable.dest;
        let c = { dest.clone() };
        let (dx, dy) = { (c.x(), c.y()) };
        dest.set_x(dx + x);
        dest.set_y(dy + y);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let mut dest = &mut self.renderable.dest;
        dest.set_x(x);
        dest.set_y(y);
    }
}

impl<'a> Sprite<'a> for DeerSprite<'a> {
    fn update(&mut self, ticks: i32) {
        let y = { self.resolve_y(&self.animatable.animation) };
        self.animatable
            .animate(&ticks, y, &mut self.renderable.source);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.renderable.render(canvas, main_renderer);
    }
}

compose_sprite!(FemaleDeerSprite, DeerSprite, DeerSprite::new_female);
compose_sprite!(MaleDeerSprite, DeerSprite, DeerSprite::new_male);
