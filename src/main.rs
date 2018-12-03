use std::path::Path;

mod day_one;
mod day_two;
mod day_three;

fn day_one() {
    let path: &Path = Path::new("input/day_1_part_1.txt");
    println!("Day 1: Part 1");
    day_one::part_one(path);
    println!("Day 1: Part 2");
    day_one::part_two(path);
}

fn day_two() {
    let path: &Path = Path::new("input/day_2_part_1.txt");
    println!("Day 2: Part 1");
    day_two::part_one(path);
    println!("Day 2: Part 2");
    day_two::part_two(path);
}

fn day_three() {
    let path: &Path = Path::new("input/day_3.txt");
    println!("Day 3: Part 1");
    day_three::part_one(path);
}

fn main() {
    let day: i32 = 3;

    match day {
        1 => day_one(),
        2 => day_two(),
        3 => day_three(),
        _ => println!("Pick a day!"),
    };
}
