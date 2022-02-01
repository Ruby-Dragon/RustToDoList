use std::env;
mod args;

fn main() {
	  let mut args: Vec<String> = env::args().collect();
		args.remove(0);
		let mut cstyleint:i32 = 20;
    println!("Hello, world! {}", cstyleint);

		let result = args::return_string(23);
		println!("{}", result);

		for arg in args
		{
			println!("{}", arg);
		}
}
