use game::map::*;
use game::map::map::*;
use game::map::layer::*;
use game::map::layer_type::*;
use game::map::tile_type::*;
use game::sprites::{Sprite, RenderPosition};
use game::sprites::ground_tile::GroundTile;
use game::sprites::female_deer_sprite::FemaleDeerSprite;
use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::sprites::map_sprite_layer::*;
use game::sprites::map_sprite_tile::*;

use std::rc::Rc;
use std::vec::Vec;

use sdl2::render::Texture;
use sdl2::rect::Rect;

pub struct MapSprite<'a> {
    pub texture: Rc<Texture<'a>>,
    pub source: Rect,
    pub dest: Rect,
    pub ground1: MapSpriteLayer<'a>,
    pub ground2: MapSpriteLayer<'a>,
    pub ground3: MapSpriteLayer<'a>,
    pub animals: MapSpriteLayer<'a>,
    pub plants: MapSpriteLayer<'a>,
    pub players: MapSpriteLayer<'a>,
    pub roofs: MapSpriteLayer<'a>,
}

impl<'a> MapSprite<'a> {
    pub fn new(map_name: &String, main_renderer: &mut MainRenderer<'a, 'a>) -> Self {
        let texture = main_renderer
            .texture_manager
            .load("./assets/textures/deer female calciumtrice.png")
            .unwrap();
        let map: Map = load_map(map_name);
//        let viewport = VisibleRange::new(
//            main_renderer.scroll.x,
//            main_renderer.scroll.y,
//            main_renderer.viewport.width as u32,
//            main_renderer.viewport.height as u32,
//            100, 100,
//        );
        let mut ground1 = MapSpriteLayer::new();
        let mut ground2 = MapSpriteLayer::new();
        let mut ground3 = MapSpriteLayer::new();
        let mut animals = MapSpriteLayer::new();
        let mut plants = MapSpriteLayer::new();
        let mut players = MapSpriteLayer::new();
        let mut roofs = MapSpriteLayer::new();

        {
            for (index, tile_type) in map.clone().ground1
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Ground1, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let x = (index as f64 / map.meta.width as f64).floor() as usize;
                let y = (index % map.meta.height) as usize;
                let mut sprite = MapSpriteTile::new();
                let mut tile_sprite = GroundTile::new(main_renderer);
                tile_sprite.set_type(&tile_type);
                tile_sprite.render_on(&x, &y);
                sprite.ground = Some(tile_sprite);
                ground1.sprites.push(sprite);
            }
            for (index, tile_type) in map.clone().ground2
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Ground2, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let x = (index as f64 / map.meta.width as f64).floor() as usize;
                let y = (index % map.meta.height) as usize;
                let mut sprite = MapSpriteTile::new();
                let mut tile_sprite = GroundTile::new(main_renderer);
                tile_sprite.set_type(&tile_type);
                tile_sprite.render_on(&x, &y);
                sprite.ground = Some(tile_sprite);
                ground2.sprites.push(sprite);
            }
            for (index, tile_type) in map.clone().ground3
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Ground3, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let x = (index as f64 / map.meta.width as f64).floor() as usize;
                let y = (index % map.meta.height) as usize;
                let mut sprite = MapSpriteTile::new();
                let mut tile_sprite = GroundTile::new(main_renderer);
                tile_sprite.set_type(&tile_type);
                tile_sprite.render_on(&x, &y);
                sprite.ground = Some(tile_sprite);
                ground3.sprites.push(sprite);
            }
            for (index, tile_type) in map.clone().animals
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Animals, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let x = (index as f64 / map.meta.width as f64).floor() as usize;
                let y = (index % map.meta.height) as usize;
                let mut sprite = MapSpriteTile::new();
                match tile_type {
                    TileType::FemaleDeer => {
                        let mut female_deer_sprite = FemaleDeerSprite::new(main_renderer);
                        female_deer_sprite.render_on(&x, &y);
                        sprite.female_deer = Some(female_deer_sprite);
                        animals.sprites.push(sprite);
                    }
                    _ => {}
                }
            }
            for (index, _tile_type) in map.clone().plants
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Plants, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let _x = (index as f64 / map.meta.width as f64).floor() as usize;
                let _y = (index % map.meta.height) as usize;
                // plants;
            }
            for (index, _tile_type) in map.clone().players
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Players, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let _x = (index as f64 / map.meta.width as f64).floor() as usize;
                let _y = (index % map.meta.height) as usize;
                // players;
            }
            for (index, _tile_type) in map.clone().roofs
                .ok_or(Ok::<Layer, TileTypeErr>(Layer { layer_type: LayerType::Roofs, tiles: Vec::new() }))
                .unwrap()
                .tiles
                .into_iter()
                .enumerate() {
                let _x = (index as f64 / map.meta.width as f64).floor() as usize;
                let _y = (index % map.meta.height) as usize;
                // roofs;
            }
        }

        MapSprite {
            texture,
            source: Rect::new(0, 0, 32, 32),
            dest: Rect::new(0, 0, 160, 160),
            ground1,
            ground2,
            ground3,
            animals,
            plants,
            players,
            roofs,
        }
    }
}

impl<'a> Sprite<'a> for MapSprite<'a> {
    fn update(&mut self, ticks: i32) {
        self.ground1.update(ticks);
        self.ground2.update(ticks);
        self.ground3.update(ticks);
        self.animals.update(ticks);
        self.plants.update(ticks);
        self.players.update(ticks);
        self.roofs.update(ticks);
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
//        let w = main_renderer.viewport.width as usize;
//        let h = main_renderer.viewport.height as usize;
        self.ground1.render(canvas, main_renderer);
        self.ground2.render(canvas, main_renderer);
        self.ground3.render(canvas, main_renderer);
        self.animals.render(canvas, main_renderer);
        self.plants.render(canvas, main_renderer);
        self.players.render(canvas, main_renderer);
        self.roofs.render(canvas, main_renderer);

    }
}
