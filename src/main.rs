#[macro_use]
extern crate clap;
extern crate nanalib;

use clap::App;
use nanalib::test;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("INPUT") {
        println!("An input file was specified");
    }
    test();
}
