pub mod female_deer_sprite;
pub mod ground_tile;
pub mod map_sprite;
pub mod map_sprite_layer;
pub mod map_sprite_tile;
pub mod quit_button;
pub mod start_button;

use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use sdl2::rect::Rect;

#[derive(Debug, Clone)]
pub enum SpriteType {
    FemaleDeer,
    MaleDeer,
}

#[derive(Debug, Clone)]
pub enum Animation {
    Idle,
    Running,
    Eating,
}

//pub enum SpriteAction {
//    Move(i32, i32),
//    Stop,
//    Eat,
//    Attack(i32, i32),
//    Die,
//}

//pub trait GameLogic {
//    fn animate(&mut self, ticks: i32, y: i32);
//
//    fn perform_action(&mut self, ticks: i32, action: &SpriteAction);
//}

pub trait Sprite<'a> {
    fn update(&mut self, ticks: i32);

    fn render(&mut self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>);
}

pub trait RenderPosition {
    fn render_on(&mut self, x: &usize, y: &usize);
}

fn check_is_inside(x: &i32, y: &i32, rect: &Rect) -> bool {
    let xs: i32 = rect.x();
    let xe: i32 = xs + rect.width() as i32;
    let ys: i32 = rect.y();
    let ye: i32 = ys + rect.width() as i32;
    (xs..xe).contains(&x) && (ys..ye).contains(&y)
}

pub fn render_on(dest: &mut Rect, tile_size: &usize, x: &usize, y: &usize) {
    dest.set_x((x * tile_size) as i32);
    dest.set_y((y * tile_size) as i32);
}
