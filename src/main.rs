use std::{fs::File, env::args};

// Purpose: Main file for the project.
use crate::ds::filter_cs::CsvFilterer;
use crate::ds::hash_map::get_map;

pub mod ds;

fn main() {
    let filterer = CsvFilterer::new(String::from("yee.csv"));
    let output = filterer.read();
    let mut m = get_map();
    // Remove a value
    m.remove("price");
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);
}
