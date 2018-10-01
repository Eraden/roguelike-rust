use std::fs::read_to_string;
use std::str::FromStr;

// 16x16
#[derive(Debug, Clone)]
pub enum TileType {
    Empty,
    //= ground
    // 7x1
    GrassSmallPlant1,
    // 7x2
    GrassSmallPlant2,
    // 7x3
    GrassSmallPlant3,
    // 7x4
    GrassSmallPlant4,
    // 7x5
    GrassSmallPlant5,
    // 7x6
    GrassSmallRock,
    // 7x7
    Grass,

    //= animals
    FemaleDeer,
    MaleDeer,
}

#[derive(Debug)]
pub enum TileTypeErr {
    InvalidTileValue(String),
}

impl FromStr for TileType {
    type Err = TileTypeErr;
    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        match contents {
            // ground
            "0" => Ok(TileType::Grass),
            "1" => Ok(TileType::GrassSmallPlant1),
            "2" => Ok(TileType::GrassSmallPlant2),
            "3" => Ok(TileType::GrassSmallPlant3),
            "4" => Ok(TileType::GrassSmallPlant4),
            "5" => Ok(TileType::GrassSmallPlant5),
            "6" => Ok(TileType::GrassSmallRock),
            // animals
            "10" => Ok(TileType::Empty),
            "11" => Ok(TileType::FemaleDeer),
            "12" => Ok(TileType::MaleDeer),
            "13" => Ok(TileType::Empty),
            "14" => Ok(TileType::Empty),
            "15" => Ok(TileType::Empty),
            // plants
            "20" => Ok(TileType::Empty),
            "21" => Ok(TileType::Empty),
            "22" => Ok(TileType::Empty),
            "23" => Ok(TileType::Empty),
            "24" => Ok(TileType::Empty),
            "25" => Ok(TileType::Empty),
            // roofs
            "30" => Ok(TileType::Empty),
            "31" => Ok(TileType::Empty),
            "32" => Ok(TileType::Empty),
            "33" => Ok(TileType::Empty),
            "34" => Ok(TileType::Empty),
            "35" => Ok(TileType::Empty),
            // players
            "60" => Ok(TileType::Empty),
            "61" => Ok(TileType::Empty),
            "62" => Ok(TileType::Empty),
            "63" => Ok(TileType::Empty),
            "64" => Ok(TileType::Empty),
            "65" => Ok(TileType::Empty),
            _ => Err(TileTypeErr::InvalidTileValue(contents.to_string().clone())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LayerType {
    Ground1,
    Ground2,
    Ground3,
    Animals,
    Plants,
    Players,
    Roofs,
}

impl LayerType {
    pub fn as_slice() -> [LayerType; 7] {
        [
            LayerType::Ground1,
            LayerType::Ground2,
            LayerType::Ground3,
            LayerType::Animals,
            LayerType::Plants,
            LayerType::Players,
            LayerType::Roofs,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            LayerType::Ground1 => "Ground1",
            LayerType::Ground2 => "Ground2",
            LayerType::Ground3 => "Ground3",
            LayerType::Plants => "Plants",
            LayerType::Animals => "Animals",
            LayerType::Players => "Players",
            LayerType::Roofs => "Roofs",
        }.to_string()
    }
}

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

#[derive(Debug)]
pub enum LayerError {
    MissingLayerMeta,
    InvalidLayerLineWidth(usize, usize),
    InvalidLayerHeight(usize, usize),
}

#[derive(Debug)]
pub struct VisibleRange {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub map_width: u32,
    pub map_height: u32,
}

impl VisibleRange {
    pub fn new(x: i32, y: i32, width: u32, height: u32, map_width: u32, map_height: u32) -> Self {
        VisibleRange {
            x,
            y,
            width,
            height,
            map_width,
            map_height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub tiles: Vec<TileType>,
    pub layer_type: LayerType,
}

impl Layer {
    pub fn new(layer_type: &LayerType, tiles: &Vec<TileType>) -> Self {
        Layer {
            layer_type: layer_type.clone(),
            tiles: tiles.clone().to_vec(),
        }
    }

    pub fn take(&self, r: &VisibleRange) -> Self {
        let mut tiles = Vec::new();
        let size = (r.width * r.height) as usize;
        tiles.reserve(size);
        (0..size).clone().collect::<Vec<usize>>().iter().for_each(|i| {
            tiles.push(self.tiles[*i + (r.y as usize * r.width as usize) + r.x as usize].clone());
        });
        Layer::new(&self.layer_type, &tiles)
    }
}

impl FromStr for Layer {
    type Err = LayerError;

    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lines: Vec<&str> = contents.split("\n").collect();
        let mut it = lines.iter();

        let layer_type = match it.next() {
            None => return Err(LayerError::MissingLayerMeta),
            Some(meta) => LayerType::as_slice().iter().find(|name| {
                let s: String = name.to_string().to_lowercase();
                let m = meta.to_string();
                let res = m.ends_with(&s);
                res
            })
                .expect(format!("To find {:?} but nothing was found\ncontents:\n{:?}", meta, contents).as_str())
                .clone(),
        };

        let mut a: Vec<TileType> = Vec::new();
        it.filter(|s| s.len() > 0).for_each(|line| {
            line
                .split::<&str>(" ")
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|n|
                    a.push(
                        n
                            .parse::<TileType>()
                            .unwrap()
                    )
                );
        });
        Ok(Layer::new(&layer_type, &a))
    }
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
        let mut meta = MapMeta { width: 0, height: 0 };
        let array: Vec<&str> = contents.split(" ")
            .collect::<Vec<&str>>();
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
        current.ground1 = self.ground1.clone().and_then(|layer| Some(layer.take(&allowed_range)));
        current.ground2 = self.ground2.clone().and_then(|layer| Some(layer.take(&allowed_range)));
        current.ground3 = self.ground3.clone().and_then(|layer| Some(layer.take(&allowed_range)));
        current.animals = self.animals.clone().and_then(|layer| Some(layer.take(&allowed_range)));
        current.plants = self.plants.clone().and_then(|layer| Some(layer.take(&allowed_range)));
        current.players = self.players.clone().and_then(|layer| Some(layer.take(&allowed_range)));
        current.roofs = self.roofs.clone().and_then(|layer| Some(layer.take(&allowed_range)));
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
        current.meta = it
            .next().unwrap()
            .parse::<MapMeta>().unwrap();

        it.filter(|s| s.len() > 0).for_each(|s| {
            s.parse::<Layer>()
                .and_then(|layer| current.set_layer(&layer)).unwrap();
        });
        Ok(current)
    }
}

pub fn load_map(name: &String) -> Map {
    let path = format!("./assets/maps/{}.map", name);
    read_to_string(&path)
        .expect(&format!("To find map in {:?} there is no such file", path))
        .parse::<Map>()
        .unwrap()
}
