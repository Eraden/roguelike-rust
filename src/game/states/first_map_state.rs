use game::sprites::female_deer_sprite::*;
use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::states::State;
use sdl2::event::Event;

pub struct FirstMapState<'a> {
    pub deers: Vec<FemaleDeerSprite<'a>>,
}

impl<'a> FirstMapState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let mut state = FirstMapState { deers: Vec::new() };

        let female_deer = FemaleDeerSprite::new(main_renderer);
        state.deers.push(female_deer);
        state
    }
}

impl<'a> State<'a> for FirstMapState<'a> {
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

    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
