use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use aoc2018::Day;

use aoc2018::get_input_for_day;

const DAY: i32 = 1;

pub fn load(days_array: &mut Vec<Day>) {
	days_array.push(Day::new(DAY, run));
}

pub fn run(input: File) {
	// Day 1 code.
    let mut reader = BufReader::new(input);
    let mut accumulator = 0;
    let mut operationCounter = 0;
    for lineResult in reader.lines() {
        if let Ok(mut line) = lineResult {
            let operation = line.get(0..1).unwrap();
            let value_string = line.get(1..).unwrap();
            let value = value_string.parse::<i32>().unwrap();
            match operation {
                "+" => accumulator += value,
                "-" => accumulator -= value,
                _ => {
                    println!("unknown operation, aborting");
                    return;
                },
            }
            operationCounter += 1;
        }
    }
    println!("Result: {}, after {} operations", accumulator, operationCounter);
}
