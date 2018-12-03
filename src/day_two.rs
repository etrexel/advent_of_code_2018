use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn get_input(path: &Path) -> Vec<String> {
    let display = path.display();
    let mut file: File = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };

    let mut raw_input: String = String::new();
    match file.read_to_string(&mut raw_input) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(raw_input) => raw_input
    };

    let split_input: Vec<&str> = raw_input.split('\n').collect();
    let split_strings: Vec<String> = split_input.into_iter().map(|word| String::from(word)).collect();
    split_strings
}

fn exactly_two_duplicates(input: &str) -> bool {
    let mut output: bool = false;
    let mut letters: [i32; 26] = [0; 26];

    for letter in input.bytes() {
        let idx: usize = (letter - 97) as usize;
        letters[idx] += 1;
    }

    for letter in letters.iter() {
        if *letter == 2 {
            output = true;
            break;
        }
    }
    output
}

fn exactly_three_duplicates(input: &str) -> bool {
    let mut output: bool = false;
    let mut letters: [i32; 26] = [0; 26];

    for letter in input.bytes() {
        let idx: usize = (letter - 97) as usize;
        letters[idx] += 1;
    }

    for letter in letters.iter() {
        if *letter == 3 {
            output = true;
            break;
        }
    }
    output
}

fn distance(first: &String, second: &String) -> i32 {
    let mut distance: i32 = 0;
    let size: i32 = first.len() as i32;
    let mut first_chars = first.chars();
    let mut second_chars = second.chars();
    for _ in 0..size {
        if first_chars.next() != second_chars.next() {
            distance += 1;
        }
    }
    distance
}

fn common_letters(first: &String, second: &String) -> String {
    let mut output: String = String::new();
    let size: i32 = first.len() as i32;
    let mut first_chars = first.chars();
    let mut second_chars = second.chars();
    for _ in 0..size {
        let current_first = first_chars.next();
        let current_second = second_chars.next();
        if current_first == current_second {
            output.push(current_first.unwrap())
        }
    }
    output
}

pub fn part_one(path: &Path) {
    let input: Vec<String> = get_input(path);

    let mut two_duplicates: i32 = 0;
    let mut three_duplicates: i32 = 0;

    for val in input {
        if exactly_two_duplicates(val.as_str()) {
            two_duplicates += 1;
        }
        if exactly_three_duplicates(val.as_str()) {
            three_duplicates += 1;
        }
    }

    println!("Checksum: {}", two_duplicates * three_duplicates);
}

pub fn part_two(path: &Path) {
    let input: Vec<String> = get_input(path);

    let mut i: usize = 0;
    let mut j: usize = i + 1;

    loop {
        if distance(&input[i], &input[j]) == 1 {
            break;
        }
        j += 1;
        if j >= input.len() {
            i += 1;
            j = i + 1;
        }
    }

    let output: String = common_letters(&input[i], &input[j]);
    println!("Common letters: {}", output);
}
