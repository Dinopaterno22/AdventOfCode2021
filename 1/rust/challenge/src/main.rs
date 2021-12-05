use std::fs::File;
use std::io::{BufRead, BufReader};

fn single_measurement() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let (mut _current, mut _count, mut _previous, mut _first): (u32, u32, u32, u8) = (0, 0, 0, 0);

    for (_index, line) in reader.lines().enumerate() {
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
    println!("Final single count: {}", _count);
}

fn triple_measurement() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let (mut _a, mut _b, mut _c, mut _current, mut _previous, mut _count, mut _first): (
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u8,
    ) = (0, 0, 0, 0, 0, 0, 0);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        _a = _b;
        _b = _c;
        _c = line.parse().unwrap();
        _previous = _current;
        _current = _a + _b + _c;
        if _current > _previous && _first > 2 {
            _count += 1;
        } else if _first < 3 {
            _first += 1;
        }
    }
    println!("Final triple count: {}", _count);
}

fn main() {
    single_measurement();
    triple_measurement();
}
