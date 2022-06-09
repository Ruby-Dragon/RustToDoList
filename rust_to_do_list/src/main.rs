//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use std::env;

mod args;
mod task;

use chrono;

mod list;
mod files;

fn main() {
    //get cli args, and remove exec name
    let mut args: Vec<String> = env::args().collect();
    //remove name of program from args
    args.remove(0);

    //figure out what to do
    args::parse_args(&args);

    /* test code
    for arg in args
    {
        println!("{}", arg);
    }
    */
}
