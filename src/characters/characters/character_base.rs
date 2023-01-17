use std::ptr::hash;

pub struct CharacterBase {
    pub(crate) hp: u32,
    //moves: [Moves; 5],
    pub(crate) name: String,
}

pub struct Moves {
    damage: u32,
    name: String,
}

impl CharacterBase {
    pub fn generate_moves (move1: String, move2: String, move3: String, move4: String, move5: String) -> Vec<Moves> {
        let mut movelist: Vec<Moves> = Vec::new();
        let punch = Moves {
            damage: 5,
            name: move1.parse().unwrap(),
        };
        let kick = Moves {
            damage: 5,
            name: move2.parse().unwrap(),
        };
        let hpunch = Moves {
            damage: 5,
            name: move3.parse().unwrap(),
        };
        let hkick = Moves {
            damage: 5,
            name: move4.parse().unwrap(),
        };
        let spec = Moves {
            damage: 5,
            name: move5.parse().unwrap(),
        };
        movelist.push(punch); movelist.push(kick); movelist.push(hpunch); movelist.push(hkick); movelist.push(spec);

        return movelist;
    }
}