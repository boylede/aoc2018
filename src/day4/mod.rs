use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::convert::From;
use std::fmt::Formatter;
use std::fmt::Display;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

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

	let grouped_records :Vec<&[Record]>= records
		.split(|record| record.activity == Activity::BeginShift)
		.filter(|record_slice| record_slice.len() > 0)
		.collect(); // ::<Vec<&[Record]>>

	// let 

	println!("found {} days worth of midnight shifts", grouped_records.len());

	let mut days :Vec<Shift>= vec!();
	let mut guards :HashMap<i32, i32> = HashMap::new();
	for day in grouped_records {
		let mut shift = Shift::new();
		let mut state = Status::Awake;
		let mut activities = day.iter();
		let mut next_activity = activities.next().unwrap(); // there is atleast 1 activity
		if let Guard::Guard(id) = next_activity.guard {
			 shift.guard = id;
		};
		shift.month = next_activity.month;
		shift.day = next_activity.day;
		// let next_minute = 
		let mut minutes = [Status::Awake; 60];
		for minute in 0..59 {
			if minute == next_activity.minute {
				state = match next_activity.activity {
					Activity::WakeUp => Status::Awake,
					Activity::FallAsleep => Status::Asleep,
					_ => panic!("input had invalid activity"),
				};
				if let Some(activity) = activities.next() {
					next_activity = activity;
				};
			}
			minutes[minute as usize] = state;
			if state == Status::Asleep {
				let nap_timer = guards.entry(shift.guard).or_insert(0);
				*nap_timer += 1;
			}
		}
		shift.minutes = minutes;
		// println!("{} = {}", shift, match guards.entry(shift.guard){
		// 	Entry::Occupied(entry) => *entry.get(),
		// 	_ => 0,
		// });
		println!("{}", shift);
		days.push(shift);
	}
	for (guard_id, nap_length) in guards.iter() {
		// let entry = match guards.entry(guard_id) {
		// 	Entry::Occupied(e) => *e.get(),
		// 	_ => 0,
		// };
		println!("Guard:{}, naps for {} minutes", guard_id, nap_length);
	}
	let (dumbo, longest_nap) = guards.iter().fold((0, 0), | (worst_guard, slept_for), (guard_id, nap_time) | {
		let mut current_guard: i32 = *guard_id;
		let mut current_nap_time: i32 = *nap_time;
		if current_nap_time < slept_for {
			current_guard = worst_guard;
			current_nap_time = slept_for;
		}
		(current_guard, current_nap_time)
	});
	

	let my_minutes : Vec<[Status; 60]>= days.iter().filter(|shift| shift.guard == dumbo).map(|shift| shift.minutes).collect();
	let total_sleep_per_minute = my_minutes.iter().fold([0;60], |total_minutes, shift_minutes| {
		let mut sum_minutes : [i32; 60] = total_minutes;
		for minute in 0..59 {
			if shift_minutes[minute as usize] == Status::Asleep {
				sum_minutes[minute as usize] += 1;
			}
		}
		sum_minutes
	});
	let mut nappiest_minute :i32 = 0;
	let mut sleep_during_nappiest_minute = 0;
	for minute in 0..59 {
		if total_sleep_per_minute[minute] > sleep_during_nappiest_minute {
			nappiest_minute = minute as i32;
			sleep_during_nappiest_minute = total_sleep_per_minute[minute];
		}
	}
	println!("The worst guard is #{}, they slept for {} minutes, but thier favourite minute to sleep was 00:{}, during which they slept on {} shifts.", dumbo, longest_nap, nappiest_minute, sleep_during_nappiest_minute);
	let answer = dumbo * nappiest_minute;
	println!("The answer was {}", answer);
}
fn part2(lines: &Vec<String>) {

}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Activity {
	BeginShift,
	FallAsleep,
	WakeUp,
	Placeholder,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Record {
	year: i32,
	month: i32,
	day: i32,
	hour: i32,
	minute: i32,
	guard: Guard,
	activity: Activity,
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
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

// #[derive(Default)]
struct Shift {
	month: i32,
	day: i32,
	guard: i32,
	minutes: [Status; 60],
}


impl Shift {
	fn new() -> Self {
		Self {
			month: 0,
			day: 0,
			guard: 0,
			minutes: [Status::Awake; 60],
		}
	}
}

impl Display for Shift {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		let mut minutes = String::new();
		for minute in self.minutes.iter() {
			minutes.push(match minute {
				Status::Awake => '.',
				Status::Asleep => '#',
			});
		}
		write!(f, "{}-{}\t#{}\t{}",
			self.month,
			self.day,
			self.guard,
			minutes
		)
	}
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
	Asleep,
	Awake,
}

// impl Default for Status {
// 	fn default() -> Status {
// 		Status::Awake
// 	}
// }