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
	let a_time = time::precise_time_ns();
	println!("loading day {} input.", DAY);
	
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
    println!("Day {} Part 1 took: {}ns", DAY, b_time - a_time);
    println!("Day {} Part 2 took: {}ns", DAY, c_time - b_time);

}


fn part1(lines: &Vec<String>) {
	let mut twice = 0;
	let mut thrice = 0;
	for l in lines {
		let line = l.to_string();
		if count_repeats(2, &line) {
			twice += 1;
		}
		if count_repeats(3, &line) {
			thrice += 1;
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
	repetition.values().any(|value| {
		*value == count
	})
}

fn count_difference(line_a: &String, line_b: &String) -> (i32, usize) {
	let mut difference = 0;
	let mut last_index = 0;
	for ((index, char_a), char_b) in line_a.char_indices().zip(line_b.chars()) {
		if char_a != char_b {
			difference += 1;
			last_index = index;
		}
	}
	(difference, last_index)
}

fn part2(lines: &Vec<String>) {
	for line_a in lines {
		for line_b in lines {
			let (difference, last_index) = count_difference(line_a, line_b);
			if difference == 1 {
				println!("found matching lines:\n{}\n{}", line_a, line_b);
				println!("remove differing character at index: {}", last_index);
				let mut my_id = line_a.clone();
				my_id.remove(last_index);
				println!("Desired ID: {}", my_id);
			}
		}
	}
}

