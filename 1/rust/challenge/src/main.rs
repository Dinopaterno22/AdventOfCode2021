use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut _current: u32 = 0;
    let mut _count: u32 = 0;
    let mut _previous: u32 = 0;
    let mut _first: u8 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if _first > 1 {
            _previous = _current;
            _current = line.parse().unwrap();
            if _current > _previous {
                _count += 1;
            }
        } else {
            _first += 1;
        }
    }
    println!("Final count: {}", _count);
}
