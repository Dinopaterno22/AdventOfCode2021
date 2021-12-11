use std::cmp::max;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

struct PointMap {
    x_size: u32,
    y_size: u32,
    step : u32,
    contents: Vec<u32>,
}

impl PointMap {
    pub fn new(x: u32, y: u32) -> Self {
        return PointMap {
            x_size: x,
            y_size: y,
            step: x,
            contents: Vec::new(),
        };
    }

    fn get_point(self, x: u32, y: u32) -> u32 {
        if y > self.y_size {
            let index: usize = (x * (y + 1)) as usize;
            return self.contents[index];
        }
        return 0;
    }
    fn set_point(&mut self, x: u32, y: u32) {
        let index: usize = (x * (y + 1)) as usize;
        self.contents[index] += 1;
    }

    fn _draw_line(&mut self, line: (Point, Point)) {
        if line.0.x - line.1.x != 0 {
            _draw_horizontal_line(self, line);
        } else if line.0.y - line.1.y != 0 {
            _draw_vertical_line(self, line);
        }
    }
    
    fn _draw_horizontal_line(&mut self, line: (Point, Point)) {
        let y = line.0.y;
    
        for i in line.0.x..line.1.x {
            self.set_point(i, y);
        }
    }
    
    fn _draw_vertical_line(&mut self, line: (Point, Point)) {
        let x = line.0.x;
    
        for i in line.0.y..line.1.y {
            self.set_point(x, i);
        }
    }
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

    for _x in 0..x_size {
        map.push(Vec::new());
    }

    for i in map.iter_mut() {
        for _y in 0..y_size {
            i.push(0);
        }
    }

    return map;
}

fn _draw_line(map: &mut PointMap, line: (Point, Point)) {
    if line.0.x - line.1.x != 0 {
        _draw_horizontal_line(map, line);
    } else if line.0.y - line.1.y != 0 {
        _draw_vertical_line(map, line);
    }
}

fn _draw_horizontal_line(map: &mut PointMap, line: (Point, Point)) {
    let y = line.0.y;

    for i in line.0.x..line.1.x {
        map.set_point(i, y);
    }
}

fn _draw_vertical_line(map: &mut PointMap, line: (Point, Point)) {
    let x = line.0.x;

    for i in line.0.y..line.1.y {
        map.set_point(x, i);
    }
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

        // Disregard diagonals
        if points.0.x != points.1.x || points.0.y != points.1.y {
            continue;
        }

        lines.push(points);
    }

    let mut map: PointMap = PointMap::new(x_size, y_size);
    for line in lines{
        _draw_line(&mut map, line);
    }
    

    Ok(())
}
