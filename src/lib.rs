extern crate reqwest;
extern crate select;

use std::fs::File;
use std::fmt;
use std::fs;
use std::io::{Write, Cursor, Seek, SeekFrom, BufRead, BufReader};

use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

use select::document::{Document};
use select::node::{Node};
use select::predicate::{Predicate, Attr, Class, Name};

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
        	get_instructions_for_day(day);
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
		download_to_file(url, &file_path);
	} else {
		println!("Found cached input file");
	}
    file_path
}

pub fn get_instructions_for_day(day: i32) {
	let file_path = format!("instructions/day{}.md", day);
	let file = fs::OpenOptions::new().read(true).write(false).create(false).open(&file_path);
	
	if let Err(_e) = file {
		let file = fs::OpenOptions::new().read(true).write(true).create(true).open(&file_path);
		if let Ok(mut file) = file {
					let doc = get_html_document(format!("https://adventofcode.com/2018/day/{}", day));
			// let mut buf = Cursor::new(Vec::with_capacity(20480));
			for main in doc.find(Name("body").descendant(Name("main"))) {
				node_to_markdown(main, &mut file);
			}
			file.flush().unwrap();
		}
	} else {
		println!("already had instructions");
	}
}

	// if let Ok(mut file) = file {
	// 	let doc = get_html_document(format!("https://adventofcode.com/2018/day/{}", day));
	// 	// let mut buf = Cursor::new(Vec::with_capacity(20480));
	// 	for main in doc.find(Name("body").descendant(Name("main"))) {
	// 		node_to_markdown(main, &mut file);
	// 	}
	// 	file.flush().unwrap();
	// } else {
	// 	println!("already had instructions");
	// }

fn node_to_markdown<W: Write>(parent: Node, buf: &mut W) {
	for node in parent.children() {
		if let Some(name) = node.name() {
			match name {
				"article" => node_to_markdown(node, buf),
				"h2" => {
					write!(buf, "## {}\n",node.text());
				},
				"p" => {
					write!(buf, "{}\n\n", node.text());
				},
				"pre" => {
					write!(buf, "\t{}\n", node.text());
				},
				"ul" => {
					write!(buf, "\n");
					node_to_markdown(node, buf);
					write!(buf, "\n");
				},
				"li" => {
					write!(buf, "  * {}\n", node.text());
				},
				_ => {
					write!(buf, "\n<{}>\n", node.text());
				},
			}
		}
	}
}

fn get_html_document(url: String) -> Document {
	let mut buf = Cursor::new(Vec::with_capacity(20480)); // 20kb buffer
	download_to_buffer(url, &mut buf);
	buf.seek(SeekFrom::Start(0));
	Document::from_read(buf).unwrap()
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

fn download_to_file(url: String,  filename: &String) {
    let mut file = match fs::OpenOptions::new().read(true).write(true).create(true).open(filename) {
    	Ok(c) => c,
        Err(e) => panic!("Error while opening file \"{}\" for writing: {}", filename, e)
    };
    download_to_buffer(url, &mut file);
}

fn download_to_buffer<W: Write>(url: String, buffer: &mut W) {
	let mut request = get_url(url);
    request.copy_to(buffer).unwrap();
    buffer.flush().unwrap();
}