use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::convert::From;
use std::fmt::Formatter;
use std::fmt::Display;

use aoc2018::Day;

const DAY: i32 = 4;


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

	// let line = lines.get(0).unwrap();
	// let tokens :Vec<&str> = line.split(|c| {
	// 	c == '[' ||
	// 	c == ']' ||
	// 	c == '-' ||
	// 	c == ' ' ||
	// 	c == ':' ||
	// 	c == '#'
	// }).collect();
	// println!("found: {}, wanted: {}", tokens.len(), tokens.get(5).unwrap());

	let records = lines.iter().map(|line| line.parse::<Record>().unwrap());
	println!("found {} records", records.len());

	// {
	// 	let early_release = records.clone().collect::<Vec<Record>>();
	// 	let first : &Record = early_release.get(0).unwrap();
	// 	println!("for example, the first one is: {}", first);
	// }

	for record in records {
		println!("{}", record);
	}
	// todo: sort records

}
fn part2(lines: &Vec<String>) {

}

enum Activity {
	BeginShift,
	FallAsleep,
	WakeUp,
	Placeholder,
}

struct Record {
	minute: i32,
	guard: Guard,
	activity: Activity,
}

enum Guard {
	Placeholder,
	Guard(i32),
	NoGuard,
}

#[derive(Debug)]
enum ParseRecordError {
	NoMinute(ParseIntError),
	NoActivity(String),
}

impl From<ParseIntError> for ParseRecordError {
	fn from(error: ParseIntError) -> Self {
        ParseRecordError::NoMinute(error)
    }
}


impl FromStr for Record {
	type Err = ParseRecordError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let tokens :Vec<&str> = s.split(|c| {
			c == '[' ||
			c == ']' ||
			c == '-' ||
			c == ' ' ||
			c == ':' ||
			c == '#'
		}).collect();

		let minute = tokens.get(5).unwrap().parse::<i32>()?;
		let mut guard = Guard::Placeholder;
		let mut activity = Activity::Placeholder;
		if let Some(token) = tokens.get(9) {
			// println!("{}", &token);
			if let Ok(guard_id) = token.parse::<i32>() {
				guard = Guard::Guard(guard_id);
				activity = Activity::BeginShift;
			} 
			// else {
			// 	let token = tokens.get(7).unwrap();
			// 	println!("{}", &token);
			// 	activity = match token {
			// 		&"" => Activity::BeginShift, // should not happen?
			// 		&"asleep" => Activity::FallAsleep,
			// 		&"up" => Activity::WakeUp,
			// 		&&_ => return Err(ParseRecordError::NoActivity(token.to_string())),
			// 	}
			// }
		} else {
			let token = tokens.get(8).unwrap();
			// println!("{}", &token);
			activity = match token {
				&"" => Activity::BeginShift, // should not happen?
				&"asleep" => Activity::FallAsleep,
				&"up" => Activity::WakeUp,
				&&_ => return Err(ParseRecordError::NoActivity(token.to_string())),
			}
		};
		Ok(Record {
			minute,
			guard,
			activity
		})
	}
}

impl Display for Record {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		let guard = match self.guard {
			Guard::Placeholder => String::from("Placeholder"),
			Guard::NoGuard => String::from("Unknown Guard"),
			Guard::Guard(id) => format!("{}", id),
		};
		write!(f, "@{} #{} {}",
			self.minute,
			guard,
			match self.activity {
				Activity::BeginShift => "Started Shift",
				Activity::WakeUp => "Woke Up",
				Activity::FallAsleep => "Fell Asleep",
				Activity::Placeholder => "??",
		})
	}
}