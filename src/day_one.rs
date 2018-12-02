use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn part_one() {
    let path: &Path = Path::new("input/day_1_part_1.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("Couldn't open input file!"),
        Ok(file) => file,
    };

    let mut all_input: String = String::new();

    match file.read_to_string(&mut all_input) {
        Err(_) => panic!("Couldn't read input file!"),
        Ok(all_input) => all_input
    };

    let strings: Vec<&str> = all_input.split('\n').collect();
    let mut frequency: i32 = 0;

    for val in strings {
        let delta = val.parse::<i32>().unwrap();
        frequency += delta;
    }

    print!("Frequency: {}", frequency);

}

pub fn part_two() {

}
