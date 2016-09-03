extern crate nom;

use std::str;
use std::collections::HashMap;

pub use nom::IResult;

#[derive(PartialEq, Debug)]
pub enum Value {
	String(Vec<u8>),
	Integer(i64),
	List(Vec<Value>),
	Dictionary(HashMap<Vec<u8>, Value>)
}

pub fn decode(src: &[u8]) -> IResult<&[u8], Value> {
	alt!(src, integer | string | list)
}
/*
pub fn encode(str: &Value) -> IResult<&[u8], Value> {

}
*/

// Specific parsing functions
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
	println!("list length: {}", val.len());
	IResult::Done(rest, Value::List(val))
}
