use game::sprites::Sprite;
use game::sprites::female_deer_sprite::*;

use game::main_renderer::MainRenderer;
use sdl2::rect::Rect;
use game::sprites::Animation;
use game::states::State;
use game::app::WindowCanvas;
use game::app::App;
use game::config::Config;
use game::events::UpdateResult;
use sdl2::event::Event;

pub struct FirstMapState<'a>
{
    pub deers: Vec<DeerSprite<'a>>,
}

impl<'a> FirstMapState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let mut state = FirstMapState {
            deers: Vec::new(),
        };

        let deer_texture = main_renderer
            .texture_manager
            .load("./assets/deer female calciumtrice.png")
            .unwrap();

        let female_deer = DeerSprite {
            texture: deer_texture,
            frames_per_animation: 5,
            tile_size: 32,
            source: Rect::new(0, 0, 32, 32),
            dest: Rect::new(0, 0, 160, 160),
            animation: Animation::Eating,
            animation_step: 0,
        };
        state.deers.push(female_deer);
        state
    }
}

impl<'a> State<'a> for FirstMapState<'a>
{
    fn update(&mut self, ticks: i32) {
        for deer in self.deers.iter_mut() {
            deer.update(ticks);
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        for deer in self.deers.iter_mut() {
            deer.render(canvas, main_renderer);
        }
    }

    fn handle_click(&mut self, event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_key_down(&mut self, event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
