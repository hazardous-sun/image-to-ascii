use std::{env};

use image_to_ascii::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    match run(&args) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}")
    }
}
