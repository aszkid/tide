extern crate tide;
use tide::meta;
use tide::bencode::Encodable;

use std::path::Path;

fn main() {
	let path = Path::new("./resources/ubuntu-16.04.1-desktop-amd64.iso.torrent");
	let torr = match meta::torrent::load_from_path(&path) {
		Ok(torr) => torr,
		Err(why) => panic!("Couldn't load torrent file: {:?}", why)
	};
	torr.print_info();

	let v = tide::bencode::Value::Integer(-546);
	let venc = v.encode().unwrap();
	println!("hello: {}", String::from_utf8(venc).unwrap());
}
