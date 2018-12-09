use std::fmt::Formatter;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;
use std::collections::HashSet;

use aoc2018::Day;

const DAY: i32 = 3;

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
	let mut cloth : HashMap<Square, i32>= HashMap::new();
	for line in lines {
		let claim = load_claim(line);
		let squares = get_claim_squares(&claim);
		for square in squares {
			let claims = cloth.entry(square).or_insert(0);
			*claims += 1;
		}
	}

	let shared = cloth.values().fold(0, |acc, x| {
		let mut res = acc;
		if *x >= 2 {
			res += 1;
		}
		res
	});
	println!("found total number of areas with multiple claims: {}", shared);
}

enum Occupation {
	Unoccupied,
	Occupied(i32),
}

fn part2(lines: &Vec<String>) {
	let mut cloth : HashMap<Square, Occupation>= HashMap::new();
	let mut claim_set : HashSet<i32> = HashSet::new();

	for line in lines {
		let claim = load_claim(line);
		let id = claim.id;
		let squares = get_claim_squares(&claim);
		let mut occupied = false;
		for square in squares {
			let prior_claims = cloth.insert(square.clone(), Occupation::Occupied(id));
			if let Some(prior_claim) = prior_claims {
				if let Occupation::Occupied(prior_id) = prior_claim {
					claim_set.remove(&prior_id);
					claim_set.remove(&id);
					occupied = true;
				}
			}
		}
		if !occupied {
			claim_set.insert(id);
		}
	}
	let free_claims = claim_set.len();
	println!("We found {} claims without conflicts:", free_claims);
	for claim in claim_set {
		println!("{}", claim);
	}
}

fn load_claim(line: &String) -> Claim {
	let mut my_claim = Claim{
		id: 0,
		left: 0,
		top: 0,
		width: 0,
		height: 0,
	};
	for (i, chunk) in line.split_whitespace().enumerate() {
		match i {
			0 => {
				//#ID
				let (_, chunk) = chunk.split_at(1);
				// println!("found id: {}", chunk);
				let id = chunk.parse::<i32>().unwrap();
				// println!("found id: {}", id);
				my_claim.id = id;
			},
			1 => {
				//Symbol @
			},
			2 => {
				//left,top:
				let mut parts :Vec<&str>= chunk.split(',').collect();
				let top = parts.pop().unwrap();
				let (top, _) = top.split_at(top.len() - 1);
				let left = parts.pop().unwrap();

				my_claim.top = top.parse::<i32>().unwrap();
				my_claim.left = left.parse::<i32>().unwrap();

			},
			3 => {
				//widthxheight
				let mut parts :Vec<&str>= chunk.split('x').collect();
				let height = parts.pop().unwrap();
				let width = parts.pop().unwrap();

				my_claim.height = height.parse::<i32>().unwrap();
				my_claim.width = width.parse::<i32>().unwrap();
			},
			_ => {
				// unexpected.
			}
		}
	}
	my_claim
}

struct Claim {
	id: i32,
	left: i32,
	top: i32,
	width: i32,
	height: i32,
}

impl Display for Claim {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "#{} @ {},{}: {}x{}\n",
			self.id,
			self.left,
			self.top,
			self.width,
			self.height
			)
	}
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Square {
	x: i32,
	y: i32,
}

fn get_claim_squares(claim: &Claim) -> Vec<Square> {
	let mut squares : Vec<Square> = vec!();
	for x in 0..claim.width {
		for y in 0..claim.height {
			squares.push(Square{
				x: x + claim.left,
				y: y + claim.top,
			});
		}
	}
	squares
}