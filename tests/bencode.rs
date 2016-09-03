extern crate tide;
use tide::bencode;
use tide::bencode::{decode, IResult, Value};

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

/*// LIST TESTS
#[test]
fn list_1() {
	assert_eq!(
		decode(b"li53e3:foo4:annee"),
		IResult::Done(&b""[..], Value::List())
	)
}*/
