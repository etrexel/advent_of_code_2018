use std::path::Path;

mod day_one;

fn main() {
    let path: &Path = Path::new("input/day_1_part_1.txt");
    println!("Day 1: Part 1");
    day_one::part_one(path);
    println!("Day 1: Part 2");
    day_one::part_two(path);
}
