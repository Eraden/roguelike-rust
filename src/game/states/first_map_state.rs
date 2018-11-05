use game::sprites::Sprite;

use game::app::WindowCanvas;
use game::events::UpdateResult;
use game::main_renderer::MainRenderer;
use game::sprites::map_sprite::MapSprite;
use game::states::State;
use sdl2::event::Event;

pub struct FirstMapState<'a> {
    pub map: MapSprite<'a>,
}

impl<'a> FirstMapState<'a> {
    pub fn new(main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        FirstMapState {
            map: MapSprite::new(&"first_map".to_string(), main_renderer),
        }
    }
}

impl<'a> State<'a> for FirstMapState<'a> {
    fn update(&mut self, ticks: i32) {
        self.map.update(ticks);
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        self.map.render(canvas, main_renderer);
    }

    fn handle_click(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn handle_key_down(&mut self, _event: &Event) -> UpdateResult {
        UpdateResult::NoOp
    }
}
