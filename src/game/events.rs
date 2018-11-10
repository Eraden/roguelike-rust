use game::sprites::Gender;
use game::sprites::PlayerClass;

#[derive(Debug, PartialEq)]
pub enum UpdateResult {
    NoOp,
    Stop,
    StartFirstMap,
    PickCharacter,
    PlayerCharacterClicked(PlayerClass, Gender),
}
