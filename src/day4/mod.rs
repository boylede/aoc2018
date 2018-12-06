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
	let records = lines.iter().map(|line| line.parse::<Record>().unwrap());
	println!("found {} records", records.len());

	let mut records = records.collect::<Vec<Record>>();
	records.sort();
	let mut last_guard_id: i32 = 0;
	for mut record in records.iter_mut() {
		// println!("{}", record);
		if let Guard::Guard(id) = record.guard {
			last_guard_id = id;
		} else {
			record.guard = Guard::Guard(last_guard_id);
		}
	}
	for record in records.iter() {
		println!("{}", record);
	}

}
fn part2(lines: &Vec<String>) {

}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Activity {
	BeginShift,
	FallAsleep,
	WakeUp,
	Placeholder,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Record {
	year: i32,
	month: i32,
	day: i32,
	hour: i32,
	minute: i32,
	guard: Guard,
	activity: Activity,
}
#[derive(Ord, PartialOrd, Eq, PartialEq)]
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


		let year = tokens.get(1).unwrap().parse::<i32>()?;
		let month = tokens.get(2).unwrap().parse::<i32>()?;
		let day = tokens.get(3).unwrap().parse::<i32>()?;
		let hour = tokens.get(4).unwrap().parse::<i32>()?;
		let minute = tokens.get(5).unwrap().parse::<i32>()?;

		let mut guard = Guard::Placeholder;
		let mut activity = Activity::Placeholder;
		// if there is a nineth token, this is a guard begin shift record
		if let Some(token) = tokens.get(9) {
			if let Ok(guard_id) = token.parse::<i32>() {
				guard = Guard::Guard(guard_id);
				activity = Activity::BeginShift;
			} 
		} else {
			// otherwise, its either falling asleep or waking up
			let token = tokens.get(8).unwrap();
			activity = match token {
				&"asleep" => Activity::FallAsleep,
				&"up" => Activity::WakeUp,
				&&_ => return Err(ParseRecordError::NoActivity(token.to_string())),
			}
		};
		Ok(Record {
			year,
			month,
			day,
			hour,
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
		write!(f, "[{}-{}-{} {}:{}] Guard #{} {}",
			self.year,
			self.month,
			self.day,
			self.hour,
			self.minute,
			guard,
			match self.activity {
				Activity::BeginShift => "begins shift",
				Activity::WakeUp => "wakes up",
				Activity::FallAsleep => "falls asleep",
				Activity::Placeholder => "??",
		})
	}
}