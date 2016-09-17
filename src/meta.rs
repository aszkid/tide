use bencode;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;
use std::str;
use std::collections::BTreeMap;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

#[derive(PartialEq, Debug)]
pub struct file {
	length: i64,       // length of file in bytes
	path: Vec<String>, // subdirectory names + file name
}

#[derive(PartialEq, Debug)]
pub struct torrent {
	announce: String,     // tracker url
	name: String,         // file (single) or directory (multi)
	piece_length: i64,    //
	pieces: Vec<Vec<u8>>, // byte-string SHA1 hashes of the pieces
	files: Vec<file>,     // files in the torrent
}

#[derive(Debug)]
pub enum MetaError {
	FileNotFound,
	FileInvalid,
	FilePermissionDenied,
	FileInterrupted,
	FileUnexpectedEof,

	FileDecodeIncomplete,
	FileDecodeError,
	FileDecodeIsNotADict,

	NoAnnounce,
	NoInfo,
	NoName,
	NoPieceLen,
	NoPieces,

	PiecesNotMul20,

	HasLenAndFiles,
	HasNoLenNoFiles,

	FileListNotDicts,
	FileListNoLen,
	FileListNoPath,

	NotAString,
	NotADict,
	NotAnInt,
	NotAList,
	InvalidUtf8,

	Other
}

impl torrent {
	pub fn load_from_path(path: &Path) -> Result<torrent, MetaError> {
		use std::io::ErrorKind;
		use bencode::IResult;

		// open
		let mut file = try!(File::open(path).map_err(|why|
			match why.kind() {
				ErrorKind::NotFound => MetaError::FileNotFound,
				ErrorKind::PermissionDenied => MetaError::FilePermissionDenied,
				_ => MetaError::Other
			}
		));

		// read to vec
		let mut raw = Vec::new();
		try!(file.read_to_end(&mut raw).map_err(|why|
			match why.kind() {
				ErrorKind::Interrupted => MetaError::FileInterrupted,
				ErrorKind::UnexpectedEof => MetaError::FileUnexpectedEof,
				ErrorKind::PermissionDenied => MetaError::FilePermissionDenied,
				_ => MetaError::Other
			}
		));

		// parse
		let decoded = bencode::decode(raw.as_slice());
		let dict = match decoded {
			IResult::Done(_, val) => {
				match val {
					bencode::Value::Dictionary(d) => d,
					_ => return Err(MetaError::FileDecodeIsNotADict)
				}
			},
			IResult::Incomplete(_) => return Err(MetaError::FileDecodeIncomplete),
			IResult::Error(_) => return Err(MetaError::FileDecodeError)
		};

		// announce url
		let torr_announce = match dict.get(&b"announce".to_vec()) {
			Some(ann) => {
				match *ann {
					bencode::Value::String(ref s) => String::from_utf8(s.clone()).unwrap(),
					_ => return Err(MetaError::NotAString)
				}
			},
			None => return Err(MetaError::NoAnnounce)
		};

		// info dictionary
		let torr_info = match dict.get(&b"info".to_vec()) {
			Some(info) => {
				match *info {
					bencode::Value::Dictionary(ref d) => d,
					_ => return Err(MetaError::NotADict)
				}
			},
			None => return Err(MetaError::NoInfo)
		};
		// name (advisory)
		let torr_name = match torr_info.get(&b"name".to_vec()) {
			Some(name) => {
				match *name {
					bencode::Value::String(ref s) => String::from_utf8(s.clone()).unwrap(),
					_ => return Err(MetaError::NotAString)
				}
			},
			None => return Err(MetaError::NoName)
		};
		// piece len
		let torr_piecelen = match torr_info.get(&b"piece length".to_vec()) {
			Some(piecelen) => {
				match *piecelen {
					bencode::Value::Integer(i) => i,
					_ => return Err(MetaError::NotAnInt)
				}
			},
			None => return Err(MetaError::NoPieceLen)
		};
		let torr_pieces = match torr_info.get(&b"pieces".to_vec()) {
			Some(pieces) => {
				match *pieces {
					bencode::Value::String(ref s) => {
						let mut temp = Vec::new();
						match split_pieces(s, &mut temp) {
							Err(why) => return Err(why),
							_ => temp
						}
					},
					_ => return Err(MetaError::NotAString)
				}
			},
			None => return Err(MetaError::NoPieces)
		};

		// 'length' and 'files' are mutually exclusive
		match torr_info.get(&b"length".to_vec()) {
			Some(len) => {
				// single-file case
				if torr_info.contains_key(&b"files".to_vec()) {
					return Err(MetaError::HasLenAndFiles)
				}
				let torr_len = match *len {
					bencode::Value::Integer(i) => i,
					_ => return Err(MetaError::NotAnInt)
				};

				let f = file {
					length: torr_len, path: vec![torr_name.clone()]
				};
				return Ok(torrent {
					announce: torr_announce,
					name: torr_name,
					piece_length: torr_piecelen,
					pieces: torr_pieces,
					files: vec![f]
				});
			},
			None => {
				// multi-file case
				let torr_files = match torr_info.get(&b"files".to_vec()) {
					Some(files) => {
						match *files {
							bencode::Value::List(ref l) => {
								let mut temp = Vec::new();
								match create_file_list(l, &mut temp) {
									Ok(()) => temp,
									Err(why) => return Err(why)
								}
							},
							_ => return Err(MetaError::NotAList)
						}
					},
					None => return Err(MetaError::HasNoLenNoFiles)
				};

				return Ok(torrent {
					announce: torr_announce,
					name: torr_name,
					piece_length: torr_piecelen,
					pieces: torr_pieces,
					files: torr_files
				});
			}
		}

	}
	/*fn load_from_magnet(url: String) -> Result<info, MetaError> {

	}*/

	// for debugging
	pub fn print_info(&self) {
		println!("Name:       {}", self.name);
		println!("Announce:   {}", self.announce);
		println!("Piece len.: {}", self.piece_length);
		println!("No. pieces: {}", self.pieces.len());
		println!("Files -----");
		for f in &self.files {
			println!("--> {}, {}", f.path.concat(), f.length);
		}
	}
}
pub fn infohash() -> String {
	let mut hasher = Sha1::new();
	hasher.input_str("hello world");
	let hex = hasher.result_str();
	hex
}

// some helper functions
fn split_pieces(raw: &Vec<u8>, to: &mut Vec<Vec<u8>>) -> Result<(), MetaError> {
	let len = raw.len();
	let mut iter = raw.iter().cloned().peekable();

	if (len % 20) != 0 {
		return Err(MetaError::PiecesNotMul20);
	}
	while iter.peek() != None {
		to.push(iter.by_ref().take(20).collect());
	}

	Ok(())
}
fn create_file_list(list: &Vec<bencode::Value>, to: &mut Vec<file>) -> Result<(), MetaError> {
	for ele in list {
		match *ele {
			bencode::Value::Dictionary(ref d) => {
				match add_file_to_list(d, to) {
					Err(why) => return Err(why),
					Ok(()) => {}
				}
			},
			_ => return Err(MetaError::FileListNotDicts)
		}
	}
	Ok(())
}
fn add_file_to_list(d: &BTreeMap<Vec<u8>, bencode::Value>, to: &mut Vec<file>) -> Result<(), MetaError> {
	let len_file = match d.get(&b"length".to_vec()) {
		Some(len) => {
			match *len {
				bencode::Value::Integer(i) => i,
				_ => return Err(MetaError::NotAnInt)
			}
		},
		None => return Err(MetaError::FileListNoLen)
	};
	let path_file = match d.get(&b"path".to_vec()) {
		Some(len) => {
			match *len {
				bencode::Value::List(ref l) => {
					let mut temp = Vec::new();
					for ele in l {
						let p = match *ele {
							bencode::Value::String(ref s) => String::from_utf8(s.clone()).unwrap(),
							_ => return Err(MetaError::NotAString)
						};
						temp.push(p);
					}
					temp
				},
				_ => return Err(MetaError::NotAList)
			}
		},
		None => return Err(MetaError::FileListNoPath)
	};

	to.push(file {
		length: len_file,
		path: path_file
	});

	Ok(())
}

// UNIT TESTING
// ----------------------------------------------
#[cfg(test)]
mod tests {

	use std::fs::File;
	use std::path::Path;
	use std::io::prelude::*;

	use super::*;

	#[test]
	fn open_1() {
		let path = Path::new("./resources/sample.torrent");
		let torr = match torrent::load_from_path(&path) {
			Ok(torr) => torr,
			Err(why) => panic!("Couldn't load torrent file: {:?}", why)
		};
		assert_eq!("sample.txt".to_owned(), torr.name);
		assert_eq!("udp://tracker.openbittorrent.com:80".to_owned(), torr.announce);
		assert_eq!(65536, torr.piece_length);
		assert_eq!(vec![file {length: 20, path: vec!["sample.txt".to_owned()]}], torr.files);
	}

	#[test]
	fn infohash_1() {
		assert_eq!(
			infohash(),
			"2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_owned()
		);
	}
}
