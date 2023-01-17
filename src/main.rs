mod characters;
use characters::characters::make_characters;

fn main() {
    println!("Hello, world! {}", make_characters().name);
}
