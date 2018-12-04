use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
    // input.seek(SeekFrom::Start(0 as u64));
    part2(&lines);
    let c_time = time::precise_time_ns();
    println!("Day 2 Part 1 took: {}ns", b_time - a_time);
    println!("Day 2 Part 2 took: {}ns", c_time - b_time);

}


fn part1(lines: &Vec<String>) {
	//
}
fn part2(lines: &Vec<String>) {
	// 
}
