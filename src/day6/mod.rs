use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashMap;


use aoc2018::Day;

const DAY: i32 = 6;

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
	let coordinates: Vec<Position> = lines.iter().map(|line| line.parse::<Position>().unwrap()).collect();
	println!("found {} input coordinates", coordinates.len());

	let mut field = Grid::with_coordinates(&coordinates);
	let mut points_info: HashMap<Point, PointInfo> = HashMap::new();
	let mut sizes: HashMap<i32, Cell> = HashMap::new();
	// let what: (i32,i32) = (0,400);
	for point in field.iter() {
		let (parent, distance):(i32, i32) = coordinates
			.iter()
			.enumerate()
			.map(|(cell, coord)| {
				let mut x_dist = coord.x - point.x;
				let mut y_dist = coord.y - point.y;
				if y_dist < 0 {
					y_dist = 0 - y_dist;
				}
				if x_dist < 0 {
					x_dist = 0 - x_dist;
				}
				(cell as i32, (x_dist + y_dist))
			})
			.fold((0,400), shortest_distance);
		// if point.x == field.min_x {
		// 	println!(" ");
		// }
		// if distance == 0 {
		// 	if parent < 10 {
		// 		print!(">");
		// 	}
		// 	print!("{}<", &parent);
		// } else {
		// 	if parent < 10 {
		// 		print!(" ");
		// 	}
		// 	print!("{} ", &parent);
		// }
		if point.x == field.min_x ||
			point.x == field.min_x + field.width ||
			point.y == field.min_y ||
			point.y == field.min_y + field.height {
				sizes.insert(parent, Cell::InfiniteArea);
			} else {
				let cell = sizes.entry(parent).or_insert(Cell::FiniteArea(0));
				if let Cell::FiniteArea(counter) = cell{
					*counter += 1;
				}
			}
		points_info.insert((*point).clone(), PointInfo{parent: parent, distance: distance});
		// point.parent = parent as i32;
		// point.distance = distance;
		// print!("[{},{}]", point.x, point.y);
	}
	let mut biggest = 0;
	for size in sizes.values() {
		if let Cell::FiniteArea(area) = size {
			if *area > biggest {
				biggest = *area;
			}
		}
	}
	println!("biggest area found: {}", biggest);
}


fn shortest_distance((best_parent, shortest_distance): (i32, i32), (parent, dist): (i32, i32)) -> (i32, i32) {
	let mut distance = dist;
	if dist < 0 {
		distance = 0 - distance;
	}
	if distance < shortest_distance {
		(parent, distance)
	} else if (distance == shortest_distance) {
		(0, distance)
	} else {
		(best_parent, shortest_distance)
	}
}

fn min_max((low, high): (i32, i32), x: i32) -> (i32, i32) {
	if x > high {
		(low, x)
	} else if x < low {
		(x, high)
	} else {
		(low, high)
	}
}

fn part2(lines: &Vec<String>) {

}


enum Cell {
	InfiniteArea,
	FiniteArea(i32),
}

// #[derive(Debug)]
// enum Point {
// 	Unevaluated(Position),
// 	EdgePoint,
// 	InternalPoint(i32), // parent 
// }
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
struct Point {
	x: i32,
	y: i32,
	// parent: i32,
	// distance: i32,
}

struct PointInfo {
	parent: i32,
	distance: i32,
}

struct Grid {
	cells: Vec<Position>,	// voronoi input points
	points: Vec<Vec<Point>>, // all the points to evaluate (e.g. the field)
	min_x: i32,				// margins
	min_y: i32,
	width: i32,
	height: i32,
}

impl Grid {
	fn iter(&self) -> GridIterator {
		GridIterator {
			grid: &self,
			current_x: 0,
			current_y: 0,
		}
	}
	// fn iter_mut(self) -> 
	// fn new(x1, y1, x2, y2) -> Self {
	// 	Self {
	// 		cells: vec!(),
	// 		points: vec!(),
	// 		min_x: x1,
	// 		min_y: y1,
	// 		width: x2 - x1,
	// 		height: y2 - y1,
	// 	}
	// }
	fn with_coordinates(coords: &Vec<Position>) -> Self {
		let (min_x, max_x) = coords.iter().map(|coord| coord.x).fold((3000,0), min_max);
		let (min_y, max_y) = coords.iter().map(|coord| coord.y).fold((3000,0), min_max);

		let mut points = vec!();
		let cells = vec!();

		let width =  max_x - min_x;
		let height =  max_y - min_y;
		let parent = 0;
		let distance = 0;
		for x in min_x..max_x {
			let mut row = vec!();
			for y in min_y..max_y {
				row.push(Point{x, y}); // , parent, distance
			}
			points.push(row);
		}

		Grid {
			cells,
			points,
			min_x,
			min_y,
			width,
			height,
		}
	}
}

// impl From<Vec<Position>> for Grid {
// 	fn from(cells: Vec<Position>) -> Self {
// 		let (min_x, max_x) = cells.iter().map(|coord| coord.x).fold((300,0), min_max);
// 		let (min_y, max_y) = cells.iter().map(|coord| coord.y).fold((300,0), min_max);

// 		let mut points = vec!();

// 		let width =  max_x - min_x;
// 		let height =  max_y - min_y;
// 		for _x in min_x..max_x {
// 			let mut row = vec!();
// 			for _y in min_y..max_y {
// 				row.push(Point::Unevaluated);
// 			}
// 			points.push(row);
// 		}

// 		Grid {
// 			cells,
// 			points,
// 			min_x,
// 			min_y,
// 			width,
// 			height,
// 		}
// 	}
// }

struct GridIterator<'a> {
	grid: &'a Grid,
	current_x: i32,
	current_y: i32,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a Point;
    fn next(&mut self) -> Option<Self::Item> {
    	let grid = self.grid;
    	if self.current_x >= grid.width {
    		self.current_x = 0;
    		self.current_y += 1;
    	}
    	let mut item = None;
    	if self.current_y >= grid.height {
    		// return None
    	} else {
	    	item = grid.points.get(self.current_x as usize)?.get(self.current_y as usize);
	    	self.current_x += 1;
    	}
    	item
	}
}

#[derive(Debug)]
struct Position {
	x: i32,
	y: i32,
}

impl FromStr for Position {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let tokens :Vec<&str> = s.split(|c| c == ',').collect();
		// println!("{:?}", tokens);
		let x = tokens.get(0).unwrap().parse::<i32>()?;
		let y = tokens.get(1).unwrap().trim_start().parse::<i32>()?;
		Ok(Position{
			x,
			y,
		})
	}
}
