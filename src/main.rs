extern crate tide;
use tide::bencode::IResult;
use tide::bencode::Value;

fn handle_val(val: &Value) {
	match *val {
		Value::Integer(ref v) => println!("Integer: {}", v),
		Value::String(ref v) => println!("String: {}", String::from_utf8(v.as_slice().to_vec()).unwrap()),
		Value::List(ref v) => {
			println!("Hello list!");
			for ele in v {
				handle_val(ele);
			}
			println!("Goodbye list!");
		},
		_ => println!("Other kinda value!"),
	}
}

fn main() {
	let dec = tide::bencode::decode(b"li3e3:fool3:bari-7eei-58ee");
	match dec {
		IResult::Done(_, val) => handle_val(&val),
		_ => println!("Not good!"),
	}
}
