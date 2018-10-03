use game::map::layer::*;
use game::map::layer_type::*;
use game::map::*;
use std::str::FromStr;

#[derive(Debug)]
pub enum MapError {
    MissingMapMeta,
    MissingMapWidth,
    MissingMapHeight,
    InvalidMapLayer,
}

#[derive(Debug)]
pub enum MapMetaError {
    InvalidMetaBlock,
    MissingMapWidth,
    MissingMapHeight,
}

#[derive(Debug, Clone)]
pub struct MapMeta {
    pub width: usize,
    pub height: usize,
}

impl MapMeta {
    pub fn new(width: usize, height: usize) -> Self {
        MapMeta { width, height }
    }
}

impl FromStr for MapMeta {
    type Err = MapMetaError;

    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut meta = MapMeta {
            width: 0,
            height: 0,
        };
        let array: Vec<&str> = contents.split(" ").collect::<Vec<&str>>();
        let mut it = array.iter();
        let meta_tag = it.next();
        if meta_tag != Some(&"'meta") {
            return Err(MapMetaError::InvalidMetaBlock);
        }
        'running: loop {
            match it.next() {
                None => break 'running,
                Some(&"w") => {
                    meta.width = match it.next() {
                        None => return Err(MapMetaError::MissingMapWidth),
                        Some(s) => s.parse::<usize>().unwrap(),
                    };
                }
                Some(&"h") => {
                    meta.height = match it.next() {
                        None => return Err(MapMetaError::MissingMapHeight),
                        Some(s) => s.to_string().parse::<usize>().unwrap(),
                    };
                }
                _ => {}
            };
        }
        match (meta.width, meta.height) {
            (0, 0) => Err(MapMetaError::MissingMapHeight),
            (0, _) => Err(MapMetaError::MissingMapWidth),
            (_, 0) => Err(MapMetaError::MissingMapHeight),
            _ => Ok(meta),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub ground1: Option<Layer>,
    pub ground2: Option<Layer>,
    pub ground3: Option<Layer>,
    pub animals: Option<Layer>,
    pub plants: Option<Layer>,
    pub players: Option<Layer>,
    pub roofs: Option<Layer>,
    pub meta: MapMeta,
}

impl Map {
    pub fn new() -> Self {
        Map {
            ground1: None::<Layer>,
            ground2: None::<Layer>,
            ground3: None::<Layer>,
            animals: None::<Layer>,
            plants: None::<Layer>,
            players: None::<Layer>,
            roofs: None::<Layer>,
            meta: MapMeta::new(0, 0),
        }
    }

    pub fn take(&self, r: &VisibleRange) -> Self {
        let meta: &MapMeta = &self.meta;
        let mut current = Map::new();
        let allowed_range = VisibleRange::new(
            self.with_max(r.x, meta.width),
            self.with_max(r.y, meta.height),
            self.with_max(r.width as i32, meta.width) as u32,
            self.with_max(r.height as i32, meta.height) as u32,
            meta.width as u32,
            meta.height as u32,
        );
        current.ground1 = self
            .ground1
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current.ground2 = self
            .ground2
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current.ground3 = self
            .ground3
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current.animals = self
            .animals
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current.plants = self
            .plants
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current.players = self
            .players
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current.roofs = self
            .roofs
            .clone()
            .and_then(|layer| Some(layer.take(&allowed_range)));
        current
    }

    fn with_max(&self, n: i32, m: usize) -> i32 {
        if n as usize >= m {
            m as i32
        } else {
            n
        }
    }

    pub fn set_layer(&mut self, val: &Layer) -> Result<&str, LayerError> {
        let res: Option<Layer> = Some(val.clone());
        match val.layer_type {
            LayerType::Ground1 => {
                self.ground1 = res;
            }
            LayerType::Ground2 => {
                self.ground2 = res;
            }
            LayerType::Ground3 => {
                self.ground3 = res;
            }
            LayerType::Animals => {
                self.animals = res;
            }
            LayerType::Players => {
                self.players = res;
            }
            LayerType::Plants => {
                self.plants = res;
            }
            LayerType::Roofs => {
                self.roofs = res;
            }
        };
        Ok("")
    }
}

impl FromStr for Map {
    type Err = MapError;

    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut current = Map::new();
        let mut lines: Vec<&str> = contents.split("\n\n").collect();

        let mut it = lines.iter_mut();
        current.meta = it.next().unwrap().parse::<MapMeta>().unwrap();

        it.filter(|s| s.len() > 0).for_each(|s| {
            s.parse::<Layer>()
                .and_then(|layer| current.set_layer(&layer))
                .unwrap();
        });
        Ok(current)
    }
}
