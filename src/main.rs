extern crate tide;
use tide::bencode::IResult;
use tide::bencode::Value;

fn handle_val(val: &Value) {
	match *val {
		Value::Integer(ref v) => println!("Integer: {}", v),
		Value::String(ref v) => println!("String: {}", String::from_utf8(*v).unwrap()),
		Value::List(ref v) => {
			for ele in v {
				handle_val(ele);
			}
		},
		_ => println!("Other kinda value!"),
	}
}

fn main() {
	let dec = tide::bencode::decode(b"li3e3:fooe");
	match dec {
		IResult::Done(_, val) => handle_val(&val),
		_ => println!("Not good!"),
	}
}
