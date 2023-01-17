mod character_base;
use crate::characters::characters::character_base::CharacterBase;

pub fn make_characters() -> Box<CharacterBase> {
    let jerry = Box::new(CharacterBase { hp: 50, name: ("jerry").parse().unwrap() });

    return jerry;
}
