extern crate getopts;

use std::env;
use std::str::FromStr;
//use std::fmt::Display;


fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("n", "name", "set name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // let provided_name = matches.opt_str("n").is_some() ? matches.opt_str("n") : "world";
    let provided_name = match matches.opt_str("n") {
        Some(s) => s,
        None => "world".to_string(),
    };

    //impl Display for Option<String>
    println!("Hello, {}!", provided_name);
}
