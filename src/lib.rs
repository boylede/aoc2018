use std::fmt;

#[derive(Debug)]
pub struct Config {
	pub input: Vec<String>,
	pub day: i32,
	pub all: bool,
}
impl Config {
	pub fn new(args: lapp::Args ) -> Result<Config, &'static str> {
		let input = args.get_strings("input");

		let all = args.get_bool("all");
		let day = args.get_integer("day");

		Ok(Config {
			input,
			all,
			day,
		})
	}
}

#[derive(Debug)]
pub struct Day {
	runner: fn(Vec<String>),
	index: i32,
}

impl Day {
	pub fn new(index: i32, runner: fn(Vec<String>)) -> Self {
		Day{runner, index}
	}
	pub fn run(self: &Self, input: Vec<String>) {
		(self.runner)(input);
	}
}

impl fmt::Display for Day {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Day #{}", self.index)
	}
}
