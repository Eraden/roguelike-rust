use sdl2::render::Texture;
use std::rc::Rc;
use game::managers::FontDetails;
use sdl2::pixels::Color;
use game::main_renderer::MainRenderer;

pub fn render_text<'a>(font_details: &FontDetails, text: &str, main_renderer: &mut MainRenderer<'a, 'a>) -> Rc<Texture<'a>> {
    let font = main_renderer.font_manager.load(&font_details).unwrap();
    let surface = font.render(text).blended(Color::RGBA(100, 100, 100, 255)).unwrap();
    Rc::new(main_renderer.texture_creator.create_texture_from_surface(&surface).unwrap())
}
