pub mod main_menu_state;
pub mod first_map_state;

use game::main_renderer::MainRenderer;
use game::app::WindowCanvas;

use sdl2::event::Event;
use game::events::UpdateResult;

pub trait State<'a> {
    fn update(&mut self, ticks: i32);
    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>);
    fn handle_click(&mut self, event: &Event) -> UpdateResult;
    fn handle_key_down(&mut self, event: &Event) -> UpdateResult;
}
