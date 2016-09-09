extern crate tide;
use tide::bencode::{decode, IResult, Value, Encodable};

use std::collections::BTreeMap;

// INTEGER TESTS
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

// ENCODING TESTS
#[test]
fn encode_1() {
	let mut enc = BTreeMap::new();
	enc.insert(b"foo".to_vec(), Value::Integer(-546));
	enc.insert(b"baroo".to_vec(), Value::List(vec![Value::String(b"nopee".to_vec()), Value::Integer(84954)]));
	let encv = Value::Dictionary(enc);

	let encs = encv.encode().unwrap();
	let dec = tide::bencode::decode(encs.as_bytes());
	match dec {
		IResult::Done(_, val) => assert_eq!(val, encv),
		IResult::Incomplete(_) => panic!("Incomplete data!"),
		IResult::Error(_) => panic!("Not good!")
	}
}
