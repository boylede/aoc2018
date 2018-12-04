use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;

use aoc2018::Day;

const DAY: i32 = 2;

pub fn load(days_array: &mut Vec<Day>) {
	days_array.push(Day::new(DAY, run));
}

pub fn run(input: File) {
	// Day 2 code.
	let a_time = time::precise_time_ns();
	println!("loading day {} input.", DAY);
	
	// let mut reader = BufReader::new(input);
	let mut lines = vec!();
	{
		let mut lines_iterator = BufReader::new(&input).lines();
	    while let Some(Ok(line)) = lines_iterator.next() {
	        lines.push(line);
	    }
	}
    let b_time = time::precise_time_ns();
    println!("Loading took: {}ns", b_time - a_time);
    post_load(lines);

}

fn post_load(lines: Vec<String>) {
    let a_time = time::precise_time_ns();
    part1(&lines);
    let b_time = time::precise_time_ns();
    part2(&lines);
    let c_time = time::precise_time_ns();
    println!("Day 2 Part 1 took: {}ns", b_time - a_time);
    println!("Day 2 Part 2 took: {}ns", c_time - b_time);

}


fn part1(lines: &Vec<String>) {
	let mut twice = 0;
	let mut thrice = 0;
	for l in lines {
		let line = l.to_string();
		let repeats_twice = count_repeats(2, &line);
		let repeats_thrice = count_repeats(3, &line);
		match (repeats_twice, repeats_thrice) {
			(true, true) => {
				// println!("Both");
				twice += 1;
				thrice += 1;
			},
			(true, false) => {
				// println!("Twice");
				twice += 1;
			},
			(false, true) => {
				// println!("Thrice");
				thrice += 1;
			},
			(false, false) => {
				// println!("Neither");
			},
		}
	}
	println!("result is {}", twice * thrice);
}

fn count_repeats(count: i32, line: &String) -> bool {
	let mut repetition : HashMap<char, i32>= HashMap::new();
	for character in line.chars() {
		let count = repetition.entry(character).or_insert(0);
		*count += 1;
	}
	// println!("{:?}", repetition);
	repetition.values().any(|value| {
		*value == count
	})
	// panic!("naw we good");
	// false
}

fn part2(lines: &Vec<String>) {
	// 
}
