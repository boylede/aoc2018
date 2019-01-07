use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry::Occupied;

use aoc2018::Day;

const DAY: i32 = 7;

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
	let prerequisites :Vec<Prerequisite> = lines.iter().map(|line| line.parse::<Prerequisite>().unwrap()).collect();
	let mut outline : HashMap<StepId, Step> = HashMap::new();
	let mut steps_seen:HashSet<StepId> = HashSet::new();
	let mut needy_steps_seen:HashSet<StepId> = HashSet::new();

	for Prerequisite{ parent, needs} in prerequisites {
		steps_seen.insert(needs);
		needy_steps_seen.insert(parent);
		let requirement = outline.entry(parent).or_insert(Step::new());
		(*requirement).add_requirement(needs);
	}

	for dangling_step in steps_seen.difference(&needy_steps_seen) {
		// println!("{:?}", dangling_step);
		outline.entry(*dangling_step).or_insert(Step::new_ready());
		// ready_steps.push(*dangling_step);
	}

	println!("{:?}", outline);
	let mut steps_to_take: Vec<StepId> = vec!();
	while outline.iter().any(|(step_id, step)| {
		step.status == StepStatus::Blocked || step.status == StepStatus::Ready
	}) {
		let mut ready_steps: Vec<StepId> = prepare_steps(&mut outline);
		ready_steps.sort();
		// print!("{}", ready_steps);
		print_steps(&ready_steps);
		ready_steps.reverse();
		if let Some(step) = ready_steps.pop() {
			println!("-> {}", &step);
			steps_to_take.push(step);
			outline.entry(step).and_modify(|entry| {
				(*entry).status = StepStatus::Completed;
			});
		}
		
	}
	// print!("\n Steps: [");
	// for step in steps_to_take {
	// 	print!("{}", step);
	// }
	// println!("]");
	print_steps(&steps_to_take);
}

fn print_steps(steps: &Vec<StepId>) {
	print!("[");
	for step in steps {
		print!("{}", step);
	}
	print!("]");
}

fn prepare_steps(outline: &mut HashMap<StepId, Step>) -> Vec<StepId> {
	let mut steps_ready : Vec<StepId> = vec!();
	for (step_id, mut step) in outline.iter() {
		if step.status == StepStatus::Ready {
			steps_ready.push(*step_id);
		} else if step.status == StepStatus::Blocked && step.needs.iter().all(|prereq| {
			let child = outline.get(prereq).unwrap();
			child.status == StepStatus::Completed
		}) {
			steps_ready.push(*step_id);
		}
	}
	for step_id in steps_ready.iter() {
		outline.entry(*step_id).and_modify(|entry| {
			(*entry).status = StepStatus::Ready;
		});
	}
	// println!("{:?}", steps_ready);
	steps_ready
}


fn part2(lines: &Vec<String>) {

}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct StepId {
	id: char,
}

impl Display for StepId {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{}", self.id)
	}
}


impl FromStr for StepId {
	type Err = ParseRecordError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(StepId{
			id: s.chars().next().unwrap(),
		})
	}
}

#[derive(Debug)]
struct Prerequisite {
	parent: StepId,
	needs: StepId,
}

#[derive(Debug)]
enum ParseRecordError {
	AnyError,
}

impl FromStr for Prerequisite {
	type Err = ParseRecordError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let tokens : Vec<&str> = s.split(' ').collect();
		let needs : StepId = tokens.get(1).unwrap().parse()?;
		let parent : StepId = tokens.get(7).unwrap().parse()?;
		Ok(Prerequisite {
			parent,
			needs,
		})
	}
}
#[derive(Debug, Eq, PartialEq)]
enum StepStatus {
	Blocked,
	Ready,
	Completed,
}
// #[derive(Debug)]
struct Step {
	pub status: StepStatus,
	pub needs: Vec<StepId>,
}

impl Step {
	fn new() -> Self {
		Step{
			status: StepStatus::Blocked,
			needs: vec!(),
		}
	}
	fn new_ready() -> Self {
		Step{
			status: StepStatus::Ready,
			needs: vec!(),
		}
	}
	fn add_requirement(&mut self, needs: StepId) {
		self.needs.push(needs);
	}
}

impl Display for Step {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		if self.status == StepStatus::Blocked {
			write!(f, "{:?}:[", self.status);
			for step in self.needs.iter() {
				write!(f, "{},", step);
			}
			write!(f, "]");
		} else {
			write!(f, "{:?}", self.status);
		}
		writeln!(f, "")
	}
}

impl Debug for Step {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{}", self)
	}
}