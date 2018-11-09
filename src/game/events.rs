use game::sprites::PlayerClass;
use game::sprites::Gender;

#[derive(Debug, PartialEq)]
pub enum UpdateResult {
    NoOp,
    Stop,
    StartFirstMap,
    PickCharacter,
    PlayerCharacterClicked(PlayerClass, Gender)
}
