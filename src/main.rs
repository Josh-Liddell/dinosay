use dinosay::Dinosay;
use std::env;

fn main() {
    if let Some(text) = env::args().nth(1) {
        text.dinosay();
    } else {
        eprintln!("Must provide text as an argument");
    }
}
