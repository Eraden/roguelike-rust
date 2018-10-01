use game::sprites::female_deer_sprite::*;
use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::states::State;
use sdl2::event::Event;
use game::sprites::map_sprite::MapSprite;

pub struct FirstMapState<'a> {
    pub deers: Vec<FemaleDeerSprite<'a>>,
    pub map: MapSprite<'a>,
}

impl<'a> FirstMapState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let mut state = FirstMapState {
            deers: Vec::new(),
            map: MapSprite::new(&"first_map".to_string(), main_renderer),
        };
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
        self.map.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.map.render(canvas, main_renderer);
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
