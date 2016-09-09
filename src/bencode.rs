extern crate nom;

use std::str;
use std::collections::BTreeMap;

pub use nom::IResult;

#[derive(PartialEq, Debug)]
pub enum Value {
	String(Vec<u8>),
	Integer(i64),
	List(Vec<Value>),
	Dictionary(BTreeMap<Vec<u8>, Value>)
}
pub trait Encodable {
	fn encode(&self) -> Result<String, &str>;
}
impl Encodable for Value {
	fn encode(&self) -> Result<String, &str> {
		match *self {
			Value::String(ref s) => Ok(encode_str(s)),
			Value::Integer(i) => Ok(encode_int(i)),
			Value::List(ref l) => Ok(encode_list(l)),
			Value::Dictionary(ref d) => Ok(encode_dict(d))
		}
	}
}
fn encode_str(s: &Vec<u8>) -> String {
	format!("{}:{}", s.len(), String::from_utf8(s.clone()).unwrap())
}
fn encode_int(i: i64) -> String {
	format!("i{}e", i.to_string())
}
fn encode_list(l: &Vec<Value>) -> String {
	let mut list = String::new();
	for ele in l {
		list.push_str(ele.encode().unwrap().as_str());
	}
	format!("l{}e", list)
}
fn encode_dict(d: &BTreeMap<Vec<u8>, Value>) -> String {
	let mut dict = String::new();
	for (k,v) in d {
		let ks = Value::String(k.clone());
		dict.push_str(ks.encode().unwrap().as_str());
		dict.push_str(v.encode().unwrap().as_str());
	}
	format!("d{}e", dict)
}

pub fn decode(src: &[u8]) -> IResult<&[u8], Value> {
	value(src)
}


// Specific parsing functions
fn value(i: &[u8]) -> IResult<&[u8], Value> {
	alt!(i, integer | string | list | dictionary)
}

fn digit(i: &[u8]) -> IResult<&[u8], u64> {
	map_res!(
		i,
		nom::digit,
		|d| str::FromStr::from_str(str::from_utf8(d).unwrap())
	)
}
fn integer(i: &[u8]) -> IResult<&[u8], Value> {
	named!(inumber<i64>, chain!(
		neg: opt!(char!('-')) ~
		n: digit,
		|| {
			match neg {
				Some(_) => -(n as i64),
				None => n as i64,
			}
		}
	));
	let (rest, val) = try_parse!(i,
		delimited!(char!('i'), inumber, char!('e'))
	);
	IResult::Done(rest, Value::Integer(val))
}

fn string(i: &[u8]) -> IResult<&[u8], Value> {
	chain!(i,
		len: digit ~
		char!(':') ~
		s: take!(len),
		|| Value::String(s.to_vec())
	)
}

fn list(i: &[u8]) -> IResult<&[u8], Value> {
	let (rest, val) = try_parse!(i,
		delimited!(char!('l'), many1!(decode), char!('e'))
	);
	IResult::Done(rest, Value::List(val))
}

fn dictionary(i: &[u8]) -> IResult<&[u8], Value> {
	chain!(i,
		char!('d') ~
		pairs: many1!(
			pair!(string, value)
		) ~
		char!('e'),
		|| {
			let mut m = BTreeMap::new();
			for p in pairs {
				// lexicographical order!!!!!!
				let k: Vec<u8>;
				match p.0 {
					Value::String(ref s) => {k = s.as_slice().to_vec()},
					_ => panic!("bencode: dictionary key is not a string!")
				};
				m.insert(k, p.1);
			}
			Value::Dictionary(m)
		}
	)
}
