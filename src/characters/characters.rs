mod character_base;
use crate::characters::characters::character_base::CharacterBase;

pub fn make_characters() -> Vec<CharacterBase> {
    let mut charlist: Vec<CharacterBase> = Vec::new();
    let jerry = CharacterBase { hp: 50, moves: CharacterBase::generate_moves("punch".parse().unwrap(), "kick".parse().unwrap(), "hpunch".parse().unwrap(), "hkick".parse().unwrap(), "spec".parse().unwrap()), name: ("jerry").parse().unwrap() };
    let jill = CharacterBase { hp: 70, moves: CharacterBase::generate_moves("punch".parse().unwrap(), "kick".parse().unwrap(), "hpunch".parse().unwrap(), "hkick".parse().unwrap(), "spec".parse().unwrap()), name: ("jill").parse().unwrap() };
    charlist.push(jerry);
    charlist.push(jill);

    return charlist;
}
