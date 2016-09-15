extern crate tide;
use tide::meta;

use std::path::Path;

fn main() {
	//let path = Path::new("./resources/ubuntu-16.04.1-desktop-amd64.iso.torrent");
	let path = Path::new("./resources/sample.torrent");
	let torr = match meta::torrent::load_from_path(&path) {
		Ok(torr) => torr,
		Err(why) => panic!("Couldn't load torrent file: {:?}", why)
	};
}
