extern crate aoc2018;
extern crate lapp;


use aoc2018::Config;
use aoc2018::Day;

use std::process;
// use std::env;

mod day1;
mod day2;


const USAGE : &'static str = "Dan Boyle's Advent of Code 2018 entries.
	-d, --day (default 1) which day's code to run.
	-a, --all run all day modules currently present.
	-i, --input (string) optional input file.
	-s, --session (string) optional session string.";

fn main() {
/* 	Load all challenges into a Vec for easy running */
	let mut days : Vec<Day>= Vec::new();

	day1::load(&mut days);
	day2::load(&mut days);

/* 	Debug Prints */
	let mut debug_text = String::new();
	debug_text.push_str("Loaded days:\n");
	for day in &days {
		debug_text.push_str(&format!("{}\n", day));
	}
	print!("{}", debug_text);

/* 	Parse Arguments */

	let mut args = lapp::Args::new(USAGE);
	match args.parse_result(){
		Ok(()) => (),
		Err(error) => {
			println!("Error parsing arguments: {}, try --help.", error);
			return
		},
	}
	let config = Config::new(args).unwrap_or_else(|err| {
		println!("{}", err);
		process::exit(0);
	});

/* 	Main Logic */
	if config.all {
		for day in days {
			println!("Running day: {}", &day);
			day.run(config.input.clone());
		}

	} else {
		let index = (config.day - 1) as usize;
		if index < days.len() {
			let day = &days[index];
			println!("Running day: {}", &day);
			day.run(config.input);
		} else {
			println!("Invalid day selection: {}", config.day);
		}

	} 

}
