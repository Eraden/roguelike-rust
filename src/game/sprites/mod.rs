#[macro_use]
pub mod prelude;
pub mod choose_character_button_sprite;
pub mod deer_sprite;
pub mod ground_tile;
pub mod map_sprite;
pub mod map_sprite_layer;
pub mod map_sprite_tile;
pub mod plant_tile;
#[macro_use]
pub mod player_sprite;
pub mod button;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerClass {
    Warrior,
    Wizard,
    Rogue,
    Ranger,
}

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
    Attacking,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
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

//pub trait Collidable {
//    fn is_collision(other: &Collidable) -> bool;
//    fn is_taken(x: &i32, y: &i32) -> bool;
//}

pub trait AnimationRowResolver {
    fn resolve_y(&self, animation: &Animation) -> i32;
}

#[derive(Clone, Debug)]
pub struct Animatable {
    animation: Animation,
    frames_per_animation: i32,
    tile_size: i32,
    animation_step: i32,
}

impl Animatable {
    pub fn new(frames_per_animation: i32, tile_size: i32) -> Self {
        Self {
            animation: Animation::Idle,
            frames_per_animation,
            tile_size,
            animation_step: 0,
        }
    }

    pub fn animate(&mut self, ticks: &i32, y: i32, source: &mut Rect) {
        let animation_step: i32 = (ticks / 300) % self.frames_per_animation;

        self.animation_step = animation_step;

        source.set_x(self.tile_size * animation_step);
        source.set_y(self.tile_size * y);
    }

    pub fn tile_size(&self) -> i32 {
        self.tile_size
    }
}

pub struct Renderable<'a> {
    texture: Rc<Texture<'a>>,
    dest_size: i32,
    pub source: Rect,
    pub dest: Rect,
}

impl<'a> Renderable<'a> {
    pub fn new(
        main_renderer: &mut MainRenderer<'a, 'a>,
        path: &str,
        dest_size: i32,
        source: Rect,
        dest: Rect,
    ) -> Self {
        Renderable {
            texture: main_renderer.texture_manager.load(path).unwrap(),
            dest_size,
            source,
            dest,
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>) {
        canvas
            .copy_ex(
                &self.texture,
                Some(self.source),
                Some(self.dest),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
    }

    pub fn dest_size(&self) -> i32 {
        self.dest_size.clone()
    }
}

pub trait ClickHandler {
    fn handle_click(&mut self, event: &Event) -> UpdateResult;
}

pub trait Sprite<'a> {
    fn update(&mut self, ticks: i32);

    fn render(&mut self, canvas: &mut WindowCanvas, _main_renderer: &mut MainRenderer<'a, 'a>);
}

pub trait RenderPosition {
    /** Move sprite by it's texture size */
    fn render_on(&mut self, x: &usize, y: &usize);

    /** Move by pixels */
    fn move_by(&mut self, x: i32, y: i32);

    /** Move to coords in pixels */
    fn move_to(&mut self, x: i32, y: i32);
}

fn check_is_inside(x: &i32, y: &i32, rect: &Rect) -> bool {
    let xs: i32 = rect.x();
    let xe: i32 = xs + rect.width() as i32;
    let ys: i32 = rect.y();
    let ye: i32 = ys + rect.width() as i32;
    xs <= *x && *x <= xe && ys <= *y && *y <= ye
}

pub fn render_on(dest: &mut Rect, tile_size: &usize, x: &usize, y: &usize) {
    dest.set_x((x * tile_size) as i32);
    dest.set_y((y * tile_size) as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detect_inside_rect() {
        assert_eq!(check_is_inside(&2, &2, &Rect::new(1, 1, 3, 3)), true);
    }

    #[test]
    fn is_detect_outside_rect() {
        assert_eq!(check_is_inside(&5, &5, &Rect::new(1, 1, 3, 3)), false);
    }

    #[test]
    fn change_render_dest() {
        let mut dest = Rect::new(0, 0, 1, 1);
        render_on(&mut dest, &1, &1, &1);
        assert_eq!(dest.x(), 1);
        assert_eq!(dest.y(), 1);
        assert_eq!(dest.width(), 1);
        assert_eq!(dest.height(), 1);
    }
}
