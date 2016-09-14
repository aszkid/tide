extern crate tide;
use tide::bencode::{decode, IResult, Value};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

	let path = Path::new("/media/elements2/internet_downloads/ubuntu-16.04.1-desktop-amd64.iso.torrent");
	let disp = path.display();
	let mut file = match File::open(&path) {
		Err(why) => panic!("Couldn't open file '{}': {}", disp, why),
		Ok(file) => file
	};

	// read file to u8 vector
	let mut s = Vec::new();
	match file.read_to_end(&mut s) {
		Err(why) => panic!("Couldn't read file '{}': {}", disp, why),
		Ok(_) => println!("Opened file '{}' correctly!", disp)
	}

	// decode file
	let dec = decode(s.as_slice());
	let dict = match dec {
		IResult::Done(_, val) => {
			println!("Parsed .torrent file!");
			match val {
				Value::Dictionary(d) => d,
				_ => panic!(".torrent file is not a dictionary!")
			}
		},
		_ => panic!("Couldn't parse torrent file!")
	};

	// get announce
	match dict.get(&b"announce".to_vec()) {
		Some(ann) => {
			match *ann {
				Value::String(ref s) => println!("Announce: {}", String::from_utf8(s.clone()).unwrap()),
				_ => panic!("Announce is not a string!")
			}
		},
		None => panic!("Torrent has no announce URL!")
	}
	match dict.get(&b"info".to_vec()) {
		Some(inforaw) => {
			match *inforaw {
				Value::Dictionary(ref info) => {
					match info.get(&b"name".to_vec()) {
						Some(nameraw) => {
							match *nameraw {
								Value::String(ref s) => println!("Name: {}", String::from_utf8(s.clone()).unwrap()),
								_ => panic!("Name is not a string!")
							}
						},
						None => panic!("No name!")
					}
				},
				_ => panic!("Info is not a dictionary!")
			}
		},
		None => panic!("Torrent has no info!")
	}
}
