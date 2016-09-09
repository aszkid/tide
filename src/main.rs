extern crate tide;
use tide::bencode::IResult;

fn main() {
	let dec = tide::bencode::decode(b"ld3:fooli32e3:bare4:annai-54231e3:bard3:per2:no3:seti456eeei987e5:mothali1ei2e5:kappaee");
	match dec {
		IResult::Done(_, val) => tide::bencode::print_val(&val),
		_ => println!("Not good!"),
	}
}
