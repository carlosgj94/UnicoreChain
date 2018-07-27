extern crate clap;
use self::clap::{App, Arg};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn cli() {
	let app = App::new("cli")
		.args_from_usage("-a, --getaddress=[STRING] 'makes an address from secret key'")
		.arg(Arg::with_name("INPUT")
			.help("the input")
			.index(1));

	let matches = app.get_matches();
	if matches.is_present("getaddress"){
		let input = matches.value_of("getaddress");
		println!("Your key is: {}", input.unwrap());
		let mut s = DefaultHasher::new();
		input.hash(&mut s);
		println!("your public address is: {}", s.finish());
	}
}
