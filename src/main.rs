#[macro_use]
extern crate clap;
extern crate nanalib;

use clap::App;
use std::fs;
use nanalib::run;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filename = matches.value_of("INPUT").unwrap();
    match fs::read_to_string(filename) {
        Ok(source) => run(&source),
        Err(_) => println!("Cannot open file: {}", filename)
    }
}
