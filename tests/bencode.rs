extern crate tide;
use tide::bencode;

#[test]
fn integer_1() {
	assert_eq!(
		bencode::decode(b"i58e"),
		bencode::IResult::Done(&b""[..], bencode::Value::Integer(58)))
}
#[test]
fn integer_2() {
	assert_eq!(
		bencode::decode(b"i-392e"),
		bencode::IResult::Done(&b""[..], bencode::Value::Integer(-392))
	)
}
#[test]
#[should_panic]
fn integer_3() {
	assert_eq!(
		bencode::decode(b"i03e"),
		bencode::IResult::Done(&b""[..], bencode::Value::Integer(3))
	)
}

#[test]
fn string_1() {
	assert_eq!(
		bencode::decode(b"11:hello world"),
		bencode::IResult::Done(&b""[..], bencode::Value::String(b"hello world".to_vec()))
	)
}
#[test]
#[should_panic]
fn string_2() {
	assert_eq!(
		bencode::decode(b"5:foo"),
		bencode::IResult::Done(&b""[..], bencode::Value::String(b"foo".to_vec()))
	)
}
