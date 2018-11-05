use game::app::WindowCanvas;
use game::main_renderer::MainRenderer;
use game::map::layer::*;
use game::map::layer_type::*;
use game::map::map::*;
use game::map::tile_type::*;
use game::map::*;
use game::sprites::female_deer_sprite::FemaleDeerSprite;
use game::sprites::ground_tile::GroundTile;
use game::sprites::map_sprite_layer::*;
use game::sprites::map_sprite_tile::*;
use game::sprites::{RenderPosition, Sprite};

use std::vec::Vec;

use sdl2::rect::Rect;
use game::sprites::male_deer_sprite::MaleDeerSprite;

enum LoadTarget {
    LoadTargetGround1,
    LoadTargetGround2,
    LoadTargetGround3,
}

pub struct MapSprite<'a> {
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
        //        let viewport = VisibleRange::new(
        //            main_renderer.scroll.x,
        //            main_renderer.scroll.y,
        //            main_renderer.viewport.width as u32,
        //            main_renderer.viewport.height as u32,
        //            100, 100,
        //        );

        let mut map_sprite = MapSprite {
            source: Rect::new(0, 0, 32, 32),
            dest: Rect::new(0, 0, 160, 160),
            ground1: MapSpriteLayer::new(),
            ground2: MapSpriteLayer::new(),
            ground3: MapSpriteLayer::new(),
            animals: MapSpriteLayer::new(),
            plants: MapSpriteLayer::new(),
            players: MapSpriteLayer::new(),
            roofs: MapSpriteLayer::new(),
        };
        map_sprite.load(map_name, main_renderer);
        map_sprite
    }

    fn load(&mut self, map_name: &String, main_renderer: &mut MainRenderer<'a, 'a>) {
        let map: Map = load_map(map_name);

        self.load_ground(&map, &map.ground1, LoadTarget::LoadTargetGround1, main_renderer);
        self.load_ground(&map, &map.ground2, LoadTarget::LoadTargetGround2, main_renderer);
        self.load_ground(&map, &map.ground3, LoadTarget::LoadTargetGround3, main_renderer);
        self.load_animals(&map, &map.animals, main_renderer);
        self.load_plants(&map, &map.plants, main_renderer);
        self.load_players(&map, &map.players, main_renderer);
        for (index, _tile_type) in map
            .clone()
            .players
            .ok_or(Ok::<Layer, TileTypeErr>(Layer {
                layer_type: LayerType::Players,
                tiles: Vec::new(),
            })).unwrap()
            .tiles
            .into_iter()
            .enumerate()
            {
                let _y = (index as f64 / map.meta.width as f64).floor() as usize;
                let _x = (index % map.meta.height) as usize;
                // players;
            }
        for (index, _tile_type) in map
            .clone()
            .roofs
            .ok_or(Ok::<Layer, TileTypeErr>(Layer {
                layer_type: LayerType::Roofs,
                tiles: Vec::new(),
            })).unwrap()
            .tiles
            .into_iter()
            .enumerate()
            {
                let _y = (index as f64 / map.meta.width as f64).floor() as usize;
                let _x = (index % map.meta.height) as usize;
                // roofs;
            }
    }

    fn load_ground(
        &mut self,
        map: &Map,
        maybe_ground: &Option<Layer>,
        target: LoadTarget,
        main_renderer: &mut MainRenderer<'a, 'a>,
    ) {
        let ground = match maybe_ground {
            Some(g) => g,
            None => return,
        };
        for index in 0..ground.tiles.len() {
            let (x, y) = self.index_to_coords(&index, map);
            let mut tile_sprite = GroundTile::new(main_renderer);
            let sprite_type = ground.tiles.get(index).unwrap();
            match sprite_type {
                TileType::Empty
                | TileType::GrassSmallPlant1
                | TileType::GrassSmallPlant2
                | TileType::GrassSmallPlant3
                | TileType::GrassSmallPlant4
                | TileType::GrassSmallPlant5
                | TileType::GrassSmallRock
                | TileType::Grass
                | TileType::PondLeftTop
                | TileType::PondLeft
                | TileType::PondLeftBottom
                | TileType::PondBottom
                | TileType::PondRightBottom
                | TileType::PondRight
                | TileType::PondRightTop
                | TileType::PondTop
                | TileType::PondWater => {
                    tile_sprite.set_type(&sprite_type);
                    tile_sprite.render_on(&x, &y);
                    let mut sprite = MapSpriteTile::new_ground_tile(tile_sprite);
                    match target {
                        LoadTarget::LoadTargetGround1 => self.ground1.sprites.push(sprite),
                        LoadTarget::LoadTargetGround2 => self.ground2.sprites.push(sprite),
                        LoadTarget::LoadTargetGround3 => self.ground3.sprites.push(sprite),
                    };
                }
                _ => {}
            };
        }
    }

    fn load_animals(&mut self, map: &Map, maybe_animals: &Option<Layer>, main_renderer: &mut MainRenderer<'a, 'a>) {
        let animals = match maybe_animals {
            Some(a) => a,
            None => return,
        };
        for (index, tile_type) in animals.tiles.iter().enumerate() {
            let (x, y) = self.index_to_coords(&index, &map);
            match tile_type {
                TileType::FemaleDeer => {
                    let mut female_deer_sprite = FemaleDeerSprite::new(main_renderer);
                    female_deer_sprite.render_on(&x, &y);
                    let mut sprite = MapSpriteTile::new_female_deer_tile(female_deer_sprite);
                    self.animals.sprites.push(sprite);
                }
                TileType::MaleDeer => {
                    let mut male_deer_sprite = MaleDeerSprite::new(main_renderer);
                    male_deer_sprite.render_on(&x, &y);
                    let mut sprite = MapSpriteTile::new_male_deer_tile(male_deer_sprite);
                    self.animals.sprites.push(sprite);
                }
                _ => {}
            }
        }
    }

    fn load_plants(&mut self, map: &Map, maybe_plants: &Option<Layer>, main_renderer: &mut MainRenderer<'a, 'a>) {
        let plants = match maybe_plants {
            Some(p) => p,
            None => return,
        };
        for (index, tile_type) in plants.tiles.iter().enumerate() {
            match tile_type {
                TileType::TinyGreenBush => {
                    let (x, y) = self.index_to_coords(&index, &map);
                    let mut tile_sprite = GroundTile::new(main_renderer);
                    tile_sprite.set_type(&tile_type);
                    tile_sprite.render_on(&x, &y);
                    let mut sprite = MapSpriteTile::new_ground_tile(tile_sprite);
                    self.plants.sprites.push(sprite);
                }
                _ => {}
            };
        }
    }

    fn load_players(&mut self, map: &Map, maybe_players: &Option<Layer>, main_renderer: &mut MainRenderer<'a, 'a>) {
        let players = match maybe_players {
            Some(a) => a,
            None => return,
        };
        for (index, tile_type) in players.tiles.iter().enumerate() {
            let (x, y) = self.index_to_coords(&index, &map);
            match tile_type {
                TileType::FemaleDeer => {
                    let mut female_deer_sprite = FemaleDeerSprite::new(main_renderer);
                    female_deer_sprite.render_on(&x, &y);
                    let mut sprite = MapSpriteTile::new_female_deer_tile(female_deer_sprite);
                    self.players.sprites.push(sprite);
                }
                _ => {}
            }
        }
    }

    fn index_to_coords(&self, index: &usize, map: &Map) -> (usize, usize) {
        let y = (*index as f64 / map.meta.width as f64).floor() as usize;
        let x = (*index % map.meta.height) as usize;
        (x, y)
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
