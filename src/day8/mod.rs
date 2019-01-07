use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt::Formatter;

use aoc2018::Day;

const DAY: i32 = 8;

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
	let license = lines.iter().next().unwrap().parse::<LicenseNode>();
	if let Ok(license) = license {
		// println!("got License: {:?}", license);
		let mut total = 0;
		license.visit_all(&mut |node: &LicenseNode| {
			for num in &node.metadata {
				total = total + num;
			}
		});
		println!("total of all metadata: {}", total);

	}
}

fn part2(lines: &Vec<String>) {

}

#[derive(Debug)]
struct LicenseNode {
	children: Vec<LicenseNode>,
	metadata: Vec<u32>,
}

impl LicenseNode {
	fn visit_all(&self, visitor: &mut FnMut(&LicenseNode) ) {
		// LicenseDepthIter {
		// 	l: self,
		// 	index: 0,
		// }
		for child in &self.children {
			child.visit_all(visitor);
		}
		visitor(&self);
	}
}

impl FromStr for LicenseNode {
	type Err = LicenseErr;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let numbers = 
			s
			.split_whitespace()
			.map(
				|s:&str| {
					s.parse::<u32>().unwrap()
				})
			.collect::<Vec<u32>>();
		Ok(parse_node(&mut numbers.iter()))
	}
}

fn parse_node(mut numbers: &mut std::slice::Iter<u32>) -> LicenseNode {
	let node_count = numbers.next().unwrap();
	let meta_count = numbers.next().unwrap();
	let mut nodes = Vec::new();
	for node_index in 0..*node_count {
		nodes.push(parse_node(&mut numbers));
	}
	let mut metas : Vec<u32> = Vec::new();
	for meta_index in 0..*meta_count {
		metas.push(*numbers.next().unwrap());
	}
	LicenseNode{
		children: nodes,
		metadata: metas,
	}
}

struct LicenseDepthIter<'a> {
	l: &'a LicenseNode,
	index: usize,
}

impl<'a> Iterator for LicenseDepthIter<'a> {
	type Item = LicenseNode;
	fn next(&mut self) -> Option<Self::Item> {
		unimplemented!()
	}
}

// struct Metadata {
// 	value: i32,
// }


// struct License {
// 	vec: Vec<u32>,
// }

enum LicenseErr {
	Error,
}
// impl FromStr for License {
// 	type Err = LicenseErr;
// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
// 		let vec : Vec<u32> = 
// 			s
// 			.split_whitespace()
// 			.map(
// 				|s:&str| {
// 					s.parse::<u32>().unwrap()
// 				})
// 			.collect::<Vec<u32>>();
// 		Ok(License {
// 			vec,
// 		})
// 	}
// }

// impl Display for License {
// 	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
// 		for number in &self.vec {
// 			write!(f, "{} ", number);
// 		}
// 		Ok(())
// 	}
// }

// impl<'a> License {
// 	fn depth_iter(&'a self) -> LicenseDepthIter {
// 		LicenseDepthIter {
// 			l: self,
// 			index: 0,
// 		}
// 	}
// 	fn example(&self) {
// 		println!("what?");
// 	}
// }

