use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn get_input(path: &Path) -> Vec<i32> {
    let display = path.display();
    let mut file: File = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };

    let mut raw_input: String = String::new();
    match file.read_to_string(&mut raw_input) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(raw_input) => raw_input
    };

    let split_input: Vec<&str> = raw_input.split('\n').collect();
    let i32_input: Vec<i32> = split_input.into_iter().map(|val| val.parse::<i32>().unwrap()).collect();
    i32_input
}

pub fn part_one(path: &Path) {
    let input: Vec<i32> = get_input(path);
    let mut frequency: i32 = 0;

    for val in input {
        frequency += val;
    }

    println!("Frequency: {}", frequency);
}

pub fn part_two(path: &Path) {
    let input = get_input(path);

    let mut idx: usize = 0;
    let mut frequency: i32 = 0;
    let duplicate: i32;
    let mut frequencies: Vec<i32> = Vec::new();
    loop {
        frequency += input[idx];
        if frequencies.contains(&frequency) {
            duplicate = frequency;
            break;
        } else {
            frequencies.push(frequency);
        }
        idx += 1;
        if idx >= input.len() {
            idx = 0;
        }
    }

    println!("Duplicate Frequency: {}", duplicate);
}
