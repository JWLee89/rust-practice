use std::{fs::File, env::args};

// Purpose: Main file for the project.
use crate::ds::filter_cs::CsvFilterer;
use crate::ds::hash_map::get_map;

pub mod ds;

fn main() {
    let filterer = CsvFilterer::new(String::from("yee.csv"));
    
}
