//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use std::env;
mod args;

fn main() {
		//get cli args, and remove exec name
	  let mut args: Vec<String> = env::args().collect();
		args.remove(0);

		//figure out what command it is, and execute it
		args::parse_args(&args);
		
		/* old test code
		for arg in args
		{
			println!("{}", arg);
		}
		*/
}
