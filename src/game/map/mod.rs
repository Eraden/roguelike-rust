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
pub enum LayerError {
    MissingLayerMeta,
    InvalidLayerLineWidth(usize, usize),
    InvalidLayerHeight(usize, usize),
}

#[derive(Debug, Clone)]
pub struct Layer {
    tiles: Vec<TileType>,
    layer_type: LayerType,
}

impl Layer {
    pub fn new(layer_type: &LayerType, tiles: &Vec<TileType>) -> Self {
        Layer {
            layer_type: layer_type.clone(),
            tiles: tiles.clone().to_vec(),
        }
    }
}

impl FromStr for Layer {
    type Err = LayerError;

    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lines: Vec<&str> = contents.split("\n").collect();
        let mut it = lines.iter();

        let layer_type = match it.next() {
            None => return Err(LayerError::MissingLayerMeta),
            Some(meta) => LayerType::as_slice().iter().find(move |name| {
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

#[derive(Debug)]
pub struct Map {
    ground1: Option<Layer>,
    ground2: Option<Layer>,
    ground3: Option<Layer>,
    animals: Option<Layer>,
    plants: Option<Layer>,
    players: Option<Layer>,
    roofs: Option<Layer>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            ground1: None,
            ground2: None,
            ground3: None,
            animals: None,
            plants: None,
            players: None,
            roofs: None,
        }
    }
}

impl FromStr for Map {
    type Err = MapError;

    fn from_str(contents: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut map = Map::new();
        let lines: Vec<&str> = contents.split("\n\n").collect();

        let mut it = lines.iter();
        let _meta = it.next().unwrap();

        it.filter(|s| s.len() > 0).for_each(|s| {
            s
                .parse::<Layer>()
                .and_then(|layer| {
                    match layer.layer_type {
                        LayerType::Ground1 => map.ground1 = Some(layer),
                        LayerType::Ground2 => map.ground2 = Some(layer),
                        LayerType::Ground3 => map.ground3 = Some(layer),
                        LayerType::Animals => map.animals = Some(layer),
                        LayerType::Players => map.players = Some(layer),
                        LayerType::Plants => map.plants = Some(layer),
                        LayerType::Roofs => map.roofs = Some(layer),
                    };
                    Ok(&"")
                }).unwrap();
        });
        Ok(map)
    }
}

pub fn load_map(name: &String) -> Map {
    let path = format!("./assets/maps/{}.map", name);
    read_to_string(&path)
        .expect(&format!("To find map in {:?} there is no such file", path))
        .parse::<Map>()
        .unwrap()
}
