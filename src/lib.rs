extern crate reqwest;

use std::io::BufRead;
use std::io::Write;
use std::fmt;
use std::fs;
use std::io::BufReader;

use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

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


pub fn get_input_for_day(day: i32) -> String {
	let file_path = format!("input/day{}.input", day);
	let file = fs::OpenOptions::new().read(true).write(false).create(false).open(&file_path);

	if let Err(_e) = file {
		println!("Downloading inputs for this day.");
		get_url_into_file("https://adventofcode.com/2018/day/1/input".to_string(), file_path);
	} else {
		println!("found input file");
	}
	// Err(NotFound) => println!("couldnt find file"),
 //    Err(e) => panic!("Error while opening file:{}", e)

    println!("loaded default file");
	unimplemented!()
}

fn make_session_header() -> HeaderMap {
	let session_file = fs::OpenOptions::new().read(true).write(false).create(false).open("session.txt").unwrap();
	let mut session_reader = BufReader::new(session_file);
	let mut session_raw = String::new();
	let len = session_reader.read_line(&mut session_raw).unwrap();
	session_raw = session_raw.trim_end().to_string();
	let mut headers = HeaderMap::new();
	if len > 0 {
		println!("using cookie: \n{}", &session_raw);
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

fn get_url_into_file(url: String,  filename: String) {
    let mut request = get_url(url);
    let mut file = match fs::OpenOptions::new().read(true).write(true).create(true).open(filename) {
    	Ok(c) => c,
        Err(e) => panic!("Error while opening file:{}", e)
    };
    request.copy_to(&mut file).unwrap();
    file.flush().unwrap();
}