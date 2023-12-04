use std::fs;

use days::day_4::part2;

pub mod days;

fn main() {
    let file = fs::read_to_string("input.txt").expect("input.txt should be valid");
    println!("{}", part2(file));
}
