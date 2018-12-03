use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

const QUILT_SIZE: usize = 1000;

struct Claim {
    id: i32,
    x_offset: i32,
    y_offset: i32,
    x_size: i32,
    y_size: i32
}

fn build_claim(input: &str) -> Claim {
    let substrings: Vec<&str> = input.split(' ').collect();
    let id_str: &str = substrings[0];
    let id: i32 = id_str[1..].parse::<i32>().unwrap();
    let offset_str: Vec<&str> = substrings[2].split(',').collect();
    let x_offset: i32 = offset_str[0].parse::<i32>().unwrap();
    let y_offset: i32 = offset_str[1][..offset_str[1].len()-1].parse::<i32>().unwrap();
    let size_str: Vec<&str> = substrings[3].split('x').collect();
    let x_size: i32 = size_str[0].parse::<i32>().unwrap();
    let y_size: i32 = size_str[1].parse::<i32>().unwrap();
    let output: Claim = Claim { id, x_offset, y_offset, x_size, y_size };
    output
}

fn get_input(path: &Path) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();

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
    for claim in split_input {
        claims.push(build_claim(claim));
    }
    claims
}

fn plot_claim(quilt: &mut Vec<Vec<i32>>, claim: &Claim) {
    let x_start: usize = claim.x_offset as usize;
    let x_end: usize = (claim.x_offset + claim.x_size) as usize;
    let y_start: usize = claim.y_offset as usize;
    let y_end: usize = (claim.y_offset + claim.y_size) as usize;

    for i in x_start..x_end {
        for j in y_start..y_end {
            quilt[i][j] += 1;
        }
    }
}

fn count_overlaps(quilt: Vec<Vec<i32>>) -> i32 {
    let mut overlaps: i32 = 0;

    for row in quilt {
        for cell in row {
            if cell >= 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

pub fn part_one(path: &Path) {
    let input: Vec<Claim> = get_input(path);

    let mut quilt: Vec<Vec<i32>> = Vec::with_capacity(QUILT_SIZE);
    for _ in 0..QUILT_SIZE {
        let mut row: Vec<i32> = Vec::with_capacity(QUILT_SIZE);
        for _i_ in 0..QUILT_SIZE {
            row.push(0);
        }
        quilt.push(row);
    }

    for claim in input {
        plot_claim(&mut quilt, &claim);
    }

    let overlaps = count_overlaps(quilt);
    println!("Overlaps: {}", overlaps);
}
