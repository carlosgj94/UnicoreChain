extern crate clap;
use clap::{App, Arg};

fn main() {
	let app = App::new("readrust")
		.version("0.1")
		.author("Diego López <lfkjalaf@gamil.com>")
		.about("Testing")
		.arg(Arg::with_name("INPUT")
			.help("the input")
			.index(1));

	let matches = app.get_matches();
	println!("{}",matches.value_of("INPUT").unwrap());
}
