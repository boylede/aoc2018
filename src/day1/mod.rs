use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;

use aoc2018::Day;

use aoc2018::get_input_for_day;

const DAY: i32 = 1;

pub fn load(days_array: &mut Vec<Day>) {
	days_array.push(Day::new(DAY, run));
}

pub fn run(optional_input:String) {
	// Day 1 code.
let mut input_filename = match optional_input.len() {
            0 => {
            	println!("using default");
            	get_input_for_day(DAY)
            },
            _ => {
            	println!("using provided input, {}", optional_input);
            	optional_input
            },
        };
    let input_file = fs::OpenOptions::new().read(true).write(false).create(false).open(input_filename);

	// if optional_input.len() > 0 {
	// 	// todo: implement parsing optional input
	// } else {
	// 	// load the default input file
	// }
}

