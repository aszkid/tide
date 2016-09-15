extern crate nom;

use std::str;
use std::collections::BTreeMap;
use std::mem;

pub use nom::IResult;

#[derive(PartialEq, Debug)]
pub enum Value {
	String(Vec<u8>),
	Integer(i64),
	List(Vec<Value>),
	Dictionary(BTreeMap<Vec<u8>, Value>)
}
pub trait Encodable {
	fn encode(&self) -> Result<Vec<u8>, &str>;
}
/*impl Encodable for Value {
	fn encode(&self) -> Result<Vec<u8>, &str> {
		match *self {
			Value::String(ref s) => Ok(encode_str(s)),
			Value::Integer(i) => Ok(encode_int(i)),
			Value::List(ref l) => Ok(encode_list(l)),
			Value::Dictionary(ref d) => Ok(encode_dict(d))
		}
	}
}
fn encode_str(s: &Vec<u8>) -> Vec<u8> {
	let mut res: Vec<u8> = Vec::new(); //format!("{}:{}", s.len(), s)
	res.push(mem::transmute(s.len()));
	res.push(mem::transmute(':'));
	res.append(&mut s.clone());
	res
}
fn encode_int(i: i64) -> Vec<u8> {
	let mut res: Vec<u8> = Vec::new(); //format!("{}:{}", s.len(), s)
	res.push(mem::transmute('i'));
	res.append(mem::transmute(i));
	res.push(mem::transmute('e'));
	res
}
fn encode_list(l: &Vec<Value>) -> Vec<u8> {
	let mut res: Vec<u8> = Vec::new();
	res.push(mem::transmute('l'));
	for ele in l {
		res.append(mem::transmute(ele.encode().unwrap()));
	}
	res.push(mem::transmute('e'));
	res
}
fn encode_dict(d: &BTreeMap<Vec<u8>, Value>) -> Vec<u8> {
	let mut res: Vec<u8> = Vec::new();
	res.push(mem::transmute('d'));
	for (k,v) in d {
		let ks = Value::String(k.clone());
		res.append(mem::transmute(ks.encode().unwrap()));
		res.append(mem::transmute(v.encode().unwrap()));
	}
	res.push(mem::transmute('e'));
	res
}*/

pub fn decode(src: &[u8]) -> IResult<&[u8], Value> {
	value(src)
}

// Just for debugging purposes
pub fn print_val(val: &Value) {
	match *val {
		Value::Integer(ref v) => println!("Integer: {}", v),
		Value::String(ref v) => println!("String: {}", String::from_utf8(v.clone()).unwrap()),
		Value::List(ref v) => {
			println!("Hello list!");
			for ele in v {
				print_val(ele);
			}
			println!("Goodbye list!");
		},
		Value::Dictionary(ref v) => {
			println!("Hello dictionary!");
			for (key, val) in v {
				println!("Key: {}", String::from_utf8(key.clone()).unwrap());
				print_val(val);
			}
			println!("Goodbye dictionary!");
		}
	}
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

// UNIT TESTING
// ----------------------------------------------
#[cfg(test)]
mod tests {

	use super::*;
	use std::collections::BTreeMap;

	#[test]
	fn integer_1() {
		assert_eq!(
			decode(b"i58e"),
			IResult::Done(&b""[..], Value::Integer(58)))
	}
	#[test]
	fn integer_2() {
		assert_eq!(
			decode(b"i-392e"),
			IResult::Done(&b""[..], Value::Integer(-392))
		)
	}
	#[test]
	#[should_panic]
	fn integer_3() {
		// Leading 0 is not valid, just like -0
		assert_eq!(
			decode(b"i03e"),
			IResult::Done(&b""[..], Value::Integer(3))
		)
	}

	// STRING TESTS
	#[test]
	fn string_1() {
		assert_eq!(
			decode(b"11:hello world"),
			IResult::Done(&b""[..], Value::String(b"hello world".to_vec()))
		)
	}
	#[test]
	#[should_panic]
	fn string_2() {
		assert_eq!(
			decode(b"5:foo"),
			IResult::Done(&b""[..], Value::String(b"foo".to_vec()))
		)
	}

	// LIST TESTS
	#[test]
	fn list_1() {
		assert_eq!(
			decode(b"li53e3:foo4:annee"),
			IResult::Done(&b""[..], Value::List(vec![
				Value::Integer(53),
				Value::String(b"foo".to_vec()),
				Value::String(b"anne".to_vec())
			]))
		)
	}

	// DICTIONARY TESTS
	#[test]
	fn dictionary_1() {
		let s = b"d3:bar4:spam3:fooi42ee";
		let mut m = BTreeMap::new();
		m.insert(b"bar".to_vec(), Value::String(b"spam".to_vec()));
		m.insert(b"foo".to_vec(), Value::Integer(42));

		assert_eq!(
			decode(s),
			IResult::Done(&b""[..], Value::Dictionary(m))
		)
	}
	#[test]
	#[should_panic]
	fn dictionary_2() {
		// Dictionaries can only have string indices
		let s = b"di2e4:spam3:fooi42ee";
		let mut m = BTreeMap::new();
		m.insert(b"2".to_vec(), Value::String(b"spam".to_vec()));
		m.insert(b"foo".to_vec(), Value::Integer(42));

		assert_eq!(
			decode(s),
			IResult::Done(&b""[..], Value::Dictionary(m))
		)
	}
	#[test]
	#[should_panic]
	fn dictionary_3() {
		// We shouldn't allow for non-lexicographically ordered dictionaries to be decoded
		assert_eq!(
			decode(b"d3:bar4:spam3:fooi42ee"),
			decode(b"d3:fooi42e3:bar4:spame")
		)
	}

	/*// ENCODING TESTS
	#[test]
	fn encode_1() {
		let mut enc = BTreeMap::new();
		enc.insert(b"foo".to_vec(), Value::Integer(-546));
		enc.insert(b"baroo".to_vec(), Value::List(vec![Value::String(b"nopee".to_vec()), Value::Integer(84954)]));
		let encv = Value::Dictionary(enc);

		let encs = encv.encode().unwrap();
		let dec = decode(encs.as_bytes());
		match dec {
			IResult::Done(_, val) => assert_eq!(val, encv),
			IResult::Incomplete(_) => panic!("Incomplete data!"),
			IResult::Error(_) => panic!("Not good!")
		}
	}*/

}
