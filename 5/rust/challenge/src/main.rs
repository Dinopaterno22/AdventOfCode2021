use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(s: &str) -> Self {
        let coord_str: Vec<&str> = s.split(",").collect();
        return Point {
            x: coord_str[0].parse::<u32>().unwrap(),
            y: coord_str[1].parse::<u32>().unwrap(),
        };
    }
}

fn split_coordinates(s: String) -> (Point, Point) {
    let split = s.split_once("->").unwrap();
    return (Point::new(split.0), Point::new(split.1));
}

fn initialize_map(x_size: u32, y_size: u32) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for x in 0..x_size {
        map.push(Vec::new());
        for y in 0..y_size {
            map[x][y].push(0);
        }
    }

    return map;
}

fn main() -> Result<(), Error> {
    let filename = "../../input_test.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let (mut x_size, mut y_size): (u32, u32) = (0, 0);
    let mut lines: Vec<(Point, Point)> = vec![];

    for line in reader.lines() {
        let line = line?;
        let points = split_coordinates(line);
        let x_max = max(points.0.x, points.1.x);
        let y_max = max(points.0.y, points.1.y);
        if x_max > x_size {
            x_size = x_max;
        }
        if y_max > y_size {
            y_size = y_max;
        }

        if points.0.x != points.1.x || points.0.y != points.1.y {
            continue;
        }

        lines.push(points);
    }

    let map: Vec<Vec<u32>>;

    Ok(())
}
