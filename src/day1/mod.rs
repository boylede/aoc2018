extern crate time;

use std::fs::File;
use std::io::prelude::*;
use std::io::{SeekFrom, Seek};
use std::io::BufReader;

use aoc2018::Day;

const DAY: i32 = 1;

pub fn load(days_array: &mut Vec<Day>) {
	days_array.push(Day::new(DAY, run));
}

pub fn run(mut input: File) {
    let a_time = time::precise_time_ns();
    part1(&input);
    let b_time = time::precise_time_ns();
    input.seek(SeekFrom::Start(0 as u64)).unwrap();
    part2(&input);
    let c_time = time::precise_time_ns();
    println!("Day 1 Part 1 took: {}ns", b_time - a_time);
    println!("Day 1 Part 2 took: {}ns", c_time - b_time);
}

fn part1(input: &File) {
	println!("Running Part 1");
    let reader = BufReader::new(input);
    let mut accumulator = 0;
    let mut operation_counter = 0;
    for line_result in reader.lines() {
        if let Ok(mut line) = line_result {
            let operation = line.get(0..1).unwrap();
            let value_string = line.get(1..).unwrap();
            let value = value_string.parse::<i32>().unwrap();
            match operation {
                "+" => accumulator += value,
                "-" => accumulator -= value,
                _ => {
                    println!("unknown operation, aborting");
                    return;
                },
            }
            operation_counter += 1;
        }
    }
    println!("Result: {}, after {} operations", accumulator, operation_counter);
}

fn part2(input: &File) {
    println!("Running Part 2");
    let reader = BufReader::new(input);
    let mut accumulator = 0;
    let mut accumulated: Vec<i32> = vec!();

    let mut lines_vec = vec!();

    let mut lines_it = reader.lines();
    while let Some(Ok(line)) = lines_it.next() {
        lines_vec.push(line);
    }

    let lines_values: Vec<i32> = lines_vec.iter().map(|line| {
        let value = line.parse::<i32>().unwrap();
        value
    }).collect();

    let values_iterator = InfiniteIter::from_vec(&lines_values);
    for value in values_iterator {
        accumulator += value;
        if accumulated.contains(&accumulator) {
            println!("Found duplicate value {}", accumulator);
            break;
        }
        accumulated.push(accumulator);
    }
    println!("Result: {}, after {} operation", accumulator, accumulated.len());

}

struct InfiniteIter<'a, T: 'a> {
    inner: &'a Vec<T>,
    pos: usize,
    loops: usize,
}

impl <'a, T> InfiniteIter<'a, T> {
    fn from_vec(vector: &'a Vec<T>) -> Self {
        Self {
            inner: vector,
            pos: 0,
            loops: 0,
        }
    }
}

impl<'a, T> Iterator for InfiniteIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.len() {
            self.pos = 0;
            self.loops += 1;
        }
        if self.loops > 200 {
        	panic!("You may have screwed up somewhere.");
        }
        let out: Option<Self::Item> = self.inner.get(self.pos);
        self.pos += 1;
        out
    }
}