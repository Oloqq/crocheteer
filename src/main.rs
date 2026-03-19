use crochet::add;

use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    println!("Add 2 + 3 = {}", add(2, 3));

    let x = Vec3::ZERO;
    let y = crochet::v0();
    let z = x + y;
    println!("compatible types yay {}", z);
}
