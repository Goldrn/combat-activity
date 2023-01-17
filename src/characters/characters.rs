mod character_base;
use crate::characters::characters::character_base::CharacterBase;

pub fn make_characters() -> Vec<CharacterBase> {
    let mut charlist: Vec<CharacterBase> = Vec::new();
    let jerry = CharacterBase { hp: 50, name: ("jerry").parse().unwrap() };
    let jill = CharacterBase { hp: 70, name: ("jill").parse().unwrap() };
    charlist.push(jerry);
    charlist.push(jill);

    return charlist;
}
