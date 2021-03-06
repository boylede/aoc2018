use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashMap;


use aoc2018::Day;

const DAY: i32 = 25;

pub fn load(days_array: &mut Vec<Day>) {
	days_array.push(Day::new(DAY, run));
}

pub fn run(input: File) {
	println!("loading day {} input.", DAY);
	let a_time = time::precise_time_ns();
	
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

}


fn part2(lines: &Vec<String>) {

}

#[derive(Debug)]
struct Point4d {
	x: i32,
	y: i32,
	z: i32,
	t: i32,
}

impl Point4d {
	fn minus(&self, b: &Point4d) -> i32 {
		distance(self.x, b.x) + distance(self.y, b.y) + distance(self.t, b.t) + distance(self.t, b.t)
	}
}

fn distance(a: i32, b: i32) -> i32{
	if a - b > 0 {
		a - b
	} else {
		b - a
	}
}