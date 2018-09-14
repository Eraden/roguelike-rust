pub mod start_button;
pub mod quit_button;
pub mod female_deer_sprite;

use game::main_renderer::MainRenderer;
use game::app::WindowCanvas;

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

    fn render(&self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>);
}
