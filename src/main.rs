mod characters;

use std::borrow::Borrow;
use characters::characters::make_characters;

fn main() {
    let chararcters = make_characters();
    let char1 = chararcters.get(1);
    let char2 = chararcters.get(0);
    println!("Hello, world! {} {}", char1.unwrap().name, char2.unwrap().name);
}
