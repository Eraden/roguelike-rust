use game::map::*;
use game::sprites::{Sprite, RenderPosition};
use game::sprites::ground_tile::GroundTile;
use game::sprites::female_deer_sprite::FemaleDeerSprite;
use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use std::rc::Rc;
use sdl2::render::Texture;
use sdl2::rect::Rect;

pub struct MapSprite<'a> {
    pub texture: Rc<Texture<'a>>,
    pub source: Rect,
    pub dest: Rect,
    pub map: Map,
    pub visible: Map,
    ground_tile: Rc<GroundTile<'a>>,
    female_deers: Vec<Rc<FemaleDeerSprite<'a>>>,
}

impl<'a> MapSprite<'a> {
    pub fn new(map_name: &String, main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let texture = main_renderer
            .texture_manager
            .load("./assets/textures/deer female calciumtrice.png")
            .unwrap();
        let map: Map = load_map(map_name);
        let viewport = VisibleRange::new(
            main_renderer.scroll.x,
            main_renderer.scroll.y,
            main_renderer.viewport.width as u32,
            main_renderer.viewport.height as u32,
            100, 100,
        );
        let visible = map.take(&viewport);
        let mut female_deers: Vec<Rc<FemaleDeerSprite>> = Vec::new();
        let animals_layer = map.clone().animals.unwrap();
        for i in 0..animals_layer.tiles.len() {
            let x = (i as f64 / map.meta.width as f64).floor() as usize;
            let y = (i % map.meta.height) as usize;
            let tile = &animals_layer.tiles[i];
            match *tile {
                TileType::FemaleDeer => {
                    let mut sprite = FemaleDeerSprite::new(main_renderer);
                    sprite.render_on(&x, &y);
                    let ptr = Rc::new(sprite);
                    female_deers.push(ptr);
                }
                _ => {}
            }
        }

        MapSprite {
            texture,
            source: Rect::new(0, 0, 32, 32),
            dest: Rect::new(0, 0, 160, 160),
            map,
            visible,
            ground_tile: Rc::new(GroundTile::new(main_renderer)),
            female_deers,
        }
    }
}

impl<'a> Sprite<'a> for MapSprite<'a> {
    fn update(&mut self, ticks: i32) {
        for i in 0..self.female_deers.len() {
            let mut wrapper = &mut self.female_deers[i];
            Rc::get_mut(wrapper)
                .unwrap()
                .update(ticks);
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas, main_renderer: &mut MainRenderer<'a, 'a>) {
        canvas.copy_ex(
            &self.texture,
            Some(self.source),
            Some(self.dest),
            0.0,
            None,
            false,
            false,
        ).unwrap();
        let w = main_renderer.viewport.width as usize;
        let h = main_renderer.viewport.height as usize;
        let map = &mut self.map;
        let ground_tile = &mut Rc::get_mut(&mut self.ground_tile)
            .unwrap();
        for y in 0..h {
            for x in 0..w {
                match map.ground1 {
                    Some(ref mut layer) => {
                        let tile_pos = (y * main_renderer.viewport.width as usize) + x;
                        let tile_type = &layer.tiles[tile_pos];
                        ground_tile.set_type(tile_type);
                        ground_tile.render_on(&x, &y);
                        ground_tile.render(canvas, main_renderer);
                    }
                    None => {}
                };
                match map.ground2 {
                    Some(ref mut layer) => {
                        let tile_pos = (y * main_renderer.viewport.width as usize) + x;
                        let tile_type = &layer.tiles[tile_pos];
                        ground_tile.set_type(tile_type);
                        ground_tile.render_on(&x, &y);
                        ground_tile.render(canvas, main_renderer);
                    }
                    None => {}
                };
                match map.ground3 {
                    Some(ref mut layer) => {
                        let tile_pos = (y * main_renderer.viewport.width as usize) + x;
                        let tile_type = &layer.tiles[tile_pos];
                        ground_tile.set_type(tile_type);
                        ground_tile.render_on(&x, &y);
                        ground_tile.render(canvas, main_renderer);
                    }
                    None => {}
                };
            }
        }
        for i in 0..self.female_deers.len() {
            let mut wrapper = &mut self.female_deers[i];
            Rc::get_mut(wrapper)
                .unwrap()
                .render(canvas, main_renderer);
        }
    }
}
