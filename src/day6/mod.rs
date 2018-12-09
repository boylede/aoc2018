use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc2018::Day;

const DAY: i32 = 6;

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
	let coordinates: Vec<Position> = lines.iter().map(|line| line.parse::<Position>().unwrap()).collect();
	println!("found {} input coordinates", coordinates.len());
	let (min_x, max_x) = coordinates.iter().map(|coord| coord.x).fold(
		(300,0), |(low_x, high_x), x| {
			// let low_x = *low_x;
			// let high_x = *high_x;
			if x > high_x {
				(low_x, x)
			} else if x < low_x {
				(x, high_x)
			} else {
				(low_x, high_x)
			}
		});
	println!("x ranges from {} to {}", min_x, max_x);
}


fn part2(lines: &Vec<String>) {

}




struct Position {
	x: i32,
	y: i32,
}

impl FromStr for Position {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let tokens :Vec<&str> = s.split(|c| c == ',').collect();
		// println!("{:?}", tokens);
		let x = tokens.get(0).unwrap().parse::<i32>()?;
		let y = tokens.get(1).unwrap().trim_start().parse::<i32>()?;
		Ok(Position{
			x,
			y,
		})
	}
}
