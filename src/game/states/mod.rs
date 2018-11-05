pub mod first_map_state;
pub mod main_menu_state;
pub mod choose_character_state;

use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;

use game::events::UpdateResult;
use sdl2::event::Event;

pub trait State<'a> {
    fn update(&mut self, ticks: i32);
    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>);
    fn handle_click(&mut self, event: &Event) -> UpdateResult;
    fn handle_key_down(&mut self, event: &Event) -> UpdateResult;
}
