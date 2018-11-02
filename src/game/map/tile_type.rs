use std::str::FromStr;

#[derive(Debug)]
pub enum TileTypeErr {
    InvalidTileValue(String),
}

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

    //- pond
    // 1x7
    PondLeftTop,
    // 1x8
    PondLeft,
    // 1x12
    PondLeftBottom,
    // 2x12
    PondBottom,
    // 6x12
    PondRightBottom,
    // 6x11
    PondRight,
    // 7x6
    PondRightTop,
    // 7x5
    PondTop,
    // 8x2
    PondWater,

    //= plants
    TinyGreenBush,
    Pine,

    //= animals
    FemaleDeer,
    MaleDeer,
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

            "7" => Ok(TileType::PondLeftTop),
            "8" => Ok(TileType::PondLeft),
            "9" => Ok(TileType::PondLeftBottom),
            "10" => Ok(TileType::PondBottom),
            "11" => Ok(TileType::PondRightBottom),
            "12" => Ok(TileType::PondRight),
            "13" => Ok(TileType::PondRightTop),
            "14" => Ok(TileType::PondTop),
            "15" => Ok(TileType::PondWater),

            // animals
            "30" => Ok(TileType::Empty),
            "31" => Ok(TileType::FemaleDeer),
            "32" => Ok(TileType::MaleDeer),
            "33" => Ok(TileType::Empty),
            "34" => Ok(TileType::Empty),
            "35" => Ok(TileType::Empty),

            // plants
            "50" => Ok(TileType::Empty),
            "51" => Ok(TileType::TinyGreenBush),
            "52" => Ok(TileType::Empty),
            "53" => Ok(TileType::Empty),
            "54" => Ok(TileType::Empty),
            "55" => Ok(TileType::Empty),

            // roofs
            "70" => Ok(TileType::Empty),
            "71" => Ok(TileType::Empty),
            "72" => Ok(TileType::Empty),
            "73" => Ok(TileType::Empty),
            "74" => Ok(TileType::Empty),
            "75" => Ok(TileType::Empty),

            // players
            "90" => Ok(TileType::Empty),
            "91" => Ok(TileType::Empty),
            "92" => Ok(TileType::Empty),
            "93" => Ok(TileType::Empty),
            "94" => Ok(TileType::Empty),
            "95" => Ok(TileType::Empty),
            _ => Err(TileTypeErr::InvalidTileValue(contents.to_string().clone())),
        }
    }
}
