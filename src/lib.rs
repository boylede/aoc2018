extern crate reqwest;

use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::fmt;
use std::fs;
use std::io::BufReader;

use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

#[derive(Debug)]
pub struct Config {
	pub day: i32,
	pub all: bool,
	pub session: String,
	pub input: String,
}
impl Config {
	pub fn new(args: lapp::Args ) -> Result<Config, &'static str> {

		let all = args.get_bool("all");
		let day = args.get_integer("day");

		let session = match args.flag_present("session") {
			true => args.get_string("session"),
			false => "session not set in command line, TODO: get from file".to_string(),
		};

		let input = match args.flag_present("input") {
			true => args.get_string("input"),
			false => "".to_string(),
		};

		Ok(Config {
			all,
			day,
			session,
			input,
		})
	}
}

#[derive(Debug)]
pub struct Day {
	runner: fn(File),
	pub index: i32,
}

impl Day {
	pub fn new(index: i32, runner: fn(File)) -> Self {
		Day{runner, index}
	}
	pub fn run(self: &Self, input: File) {
		(self.runner)(input);
	}
}

impl fmt::Display for Day {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Day #{}", self.index)
	}
}

pub fn get_input_file(day: i32, optional_input: &String) -> Result<File, std::io::Error> {
	let input_filename = match optional_input.len() {
        0 => {
        	println!("Using default input");
        	get_input_for_day(day)
        },
        _ => {
        	println!("Using user-provided input, {}", optional_input);
        	optional_input.clone()
        },
    };
    fs::OpenOptions::new().read(true).write(false).create(false).open(input_filename)
}


pub fn get_input_for_day(day: i32) -> String {
	let file_path = format!("input/day{}.txt", day);
	let file = fs::OpenOptions::new().read(true).write(false).create(false).open(&file_path);
	let url = format!("https://adventofcode.com/2018/day/{}/input", day);
	if let Err(_e) = file {
		println!("Downloading inputs for this day.");
		get_url_into_file(url, &file_path);
	} else {
		println!("Found cached input file");
	}
    file_path
}

fn make_session_header() -> HeaderMap {
	let session_file = fs::OpenOptions::new().read(true).write(false).create(false).open("session.txt").unwrap();
	let mut session_reader = BufReader::new(session_file);
	let mut session_raw = String::new();
	let len = session_reader.read_line(&mut session_raw).unwrap();
	session_raw = session_raw.trim_end().to_string();
	let mut headers = HeaderMap::new();
	if len > 0 {
		let name = HeaderName::from_lowercase(b"cookie").unwrap();
		let value = match HeaderValue::from_str(session_raw.trim_end()) {
			Ok(c) => c,
			Err(e) => panic!("Error with your session.txt file, {}", e),
		};
		headers.insert(
			name,
			value
			);
	}
	headers
}

fn get_url(url:String) -> reqwest::Response {
	let headers = make_session_header();
	let request = reqwest::Client::new().get(&url).headers(headers).send();

    let res = match request{
            Ok(c) => c,
            Err(e) => panic!("err:{}", e)
        };
    res
}

fn get_url_into_file(url: String,  filename: &String) {
    let mut file = match fs::OpenOptions::new().read(true).write(true).create(true).open(filename) {
    	Ok(c) => c,
        Err(e) => panic!("Error while opening file \"{}\" for writing: {}", filename, e)
    };
    let mut request = get_url(url);
    request.copy_to(&mut file).unwrap();
    file.flush().unwrap();
}