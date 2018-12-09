extern crate time;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

use aoc2018::Day;

const DAY: i32 = 1;

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
    let value = lines.iter().map(|line| {
        line.parse::<i32>().unwrap()
    }).fold(0, |acc, value|{
        acc + value
    });
    println!("Part 1 Result: {}", value);
}

fn part2(lines: &Vec<String>) {
    let values : Vec<i32> = lines.iter().map(|line| {
        line.parse::<i32>().unwrap()
    }).collect();
    let values_iterator = InfiniteIter::from_vec(&values);

    let mut accumulator = 0;
    let mut accumulated: HashSet<i32> = HashSet::new();

    for value in values_iterator {
        accumulator += value;
        if accumulated.contains(&accumulator) {
            println!("Found duplicate value {}", accumulator);
            break;
        }
        accumulated.insert(accumulator);
    }
    println!("Part 2 Result: {}", accumulator);

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