pub struct CharacterBase {
    pub(crate) hp: u32,
    //moves: [Moves; 5],
    pub(crate) name: String,
}

struct Moves {
    damage: u32,
    name: String,
}