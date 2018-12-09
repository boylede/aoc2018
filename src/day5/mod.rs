use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::char;
use std::slice;

use aoc2018::Day;

const DAY: i32 = 5;

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
	for line in lines {
		if line.len() > 1 {
			solve_polymer(line.clone());
		}
	}
}

fn solve_polymer(line_ref: String) -> i32 {
	let (mut reactions, mut polymer) = reduce_polymer(line_ref.clone());
	while (reactions > 0) {
		let (count, new_polymer) = reduce_polymer(polymer.clone());
		reactions = count;
		polymer = new_polymer;
	}
	println!("answer: reduced input to {} chars", polymer.len());
	polymer.len() as i32
}

fn reduce_polymer(polymer: String) -> (usize, String) {
	let first = polymer.chars();
	let second = polymer.chars().skip(1);
	let third = polymer.chars().skip(2);

	let mut reactions :Vec<usize> = vec!();
	for (i, (current, (prev, next))) in second.zip(first.zip(third)).enumerate() {
			let toggled_char = match current.is_lowercase() {
				true => current.to_ascii_uppercase(),
				false => current.to_ascii_lowercase(),
			};
			if toggled_char == prev {
				if toggled_char != next {
					// println!("{}: can match with previous char: {}, {}", i, current, prev);
					reactions.push(i);
				}
			}
		}
		// println!("found {} reactions, out of {} characters total", reactions.len(), polymer.len());
		let mut new_polymer = polymer.clone();
		for (i, index) in reactions.iter().enumerate() {
			let removal = index - (i*2);
			new_polymer.remove(removal);
			new_polymer.remove(removal);
		}
		// println!("{}->{}", reactions.len() * 2, new_polymer.len());
		(reactions.len(), new_polymer)
}


// fn diesolvePolymer(line: String) {
// 	let mut reactions: i32 = 1;
// 	while reactions > 0 {
// 		for i in line.len() {
// 			//
// 			if line.get(i);
// 		}
// 		reactions = 0;
// 	}
// }

fn part2(lines: &Vec<String>) {
	for line in lines {
		if line.len() > 1 {
			let mut results: Vec<(char, i32)> = vec!();
			for letter in 0..26 {
				let character = char::from_u32(letter + 65).unwrap(); // 65 = A, 66 = B.. etc
				let lower_character = character.to_ascii_lowercase();
				print!("trying {} -> ", character);
				let input_without_char : String = line
					.clone()
					.chars()
					.filter(|reagent| *reagent != character && *reagent != lower_character)
					// .cloned()
					.collect::<String>();
					// .to_string();
				let result = solve_polymer(input_without_char);
				results.push((character, result));
			}
			let (best_result, length) = results.iter().fold(('a', line.len() as i32), |(best_char, best_length): (char, i32), (current_char, current_length): &(char, i32)| {
				if *current_length < best_length {
					(*current_char, *current_length)
				} else {
					(best_char, best_length)
				}
			});
			println!("tried {} versions. best result was {}@{}", results.len(), best_result, length);
		}
	}
}
