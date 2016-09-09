extern crate tide;
use tide::bencode::{IResult, Value};

fn handle_val(val: &Value) {
	match *val {
		Value::Integer(ref v) => println!("Integer: {}", v),
		Value::String(ref v) => println!("String: {}", String::from_utf8(v.clone()).unwrap()),
		Value::List(ref v) => {
			println!("Hello list!");
			for ele in v {
				handle_val(ele);
			}
			println!("Goodbye list!");
		},
		Value::Dictionary(ref v) => {
			println!("Hello dictionary!");
			for (key, val) in v {
				println!("Key: {}", String::from_utf8(key.clone()).unwrap());
				handle_val(val);
			}
			println!("Goodbye dictionary!");
		}
	}
}

fn main() {
	let dec = tide::bencode::decode(b"ld3:fooli32e3:bare4:annai-54231e3:bard3:per2:no3:seti456eeei987e5:mothali1ei2e5:kappaee");
	match dec {
		IResult::Done(_, val) => handle_val(&val),
		_ => println!("Not good!"),
	}
}
