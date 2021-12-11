use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

struct PointMap {
    _x_size: u32,
    _y_size: u32,
    _step: u32,
    contents: Vec<u32>,
}

impl PointMap {
    pub fn new(x: u32, y: u32) -> Self {
        let mut content_vec: Vec<u32> = Vec::new();
        let size: u32 = x * y;
        for _i in 0..size {
            content_vec.push(0);
        }
        PointMap {
            _x_size: x,
            _y_size: y,
            _step: x,
            contents: content_vec,
        }
    }

    fn _get_point(self, x: u32, y: u32) -> u32 {
        if y > self._y_size {
            let index: usize = (x + (y * self._step)) as usize;
            return self.contents[index];
        }
        0
    }
    fn set_point(&mut self, x: u32, y: u32) {
        let index: usize = (x + (y * self._step)) as usize;
        self.contents[index] += 1;
    }

    fn draw_line(&mut self, line: (Point, Point)) {
        let horizontal: bool = ((line.0.x as i32) - (line.1.x as i32)) != 0;
        let vertical: bool = ((line.0.y as i32) - (line.1.y as i32)) != 0;
        if vertical && horizontal {
            self.draw_diagonal_line(line);
        } else if horizontal {
            self.draw_horizontal_line(line);
        } else if vertical {
            self.draw_vertical_line(line);
        }
    }

    fn draw_diagonal_line(&mut self, line: (Point, Point)) {
        // Determine Quadrant
        let horizontal_decrease: bool = ((line.0.x as i32) - (line.1.x as i32)) > 0;
        let vertical_decrease: bool = ((line.0.y as i32) - (line.1.y as i32)) > 0;
        
        let num_iter : u32 = ((line.0.x as i32) - (line.1.x as i32)).abs() as u32;

        let mut x : u32 = line.0.x;
        let mut y : u32 = line.0.y;
        for _i in 0..num_iter{
            self.set_point(x, y);
            if vertical_decrease{
                y -= 1;
            }
            else{
                y+=1;
            }
            if horizontal_decrease{
                x -= 1;
            }
            else{
                x += 1;
            }
        }
        self.set_point(x,y);
    }

    fn draw_horizontal_line(&mut self, line: (Point, Point)) {
        let y = line.0.y;
        let min = min(line.0.x, line.1.x);
        let max = max(line.0.x, line.1.x);
        for i in min..=max {
            self.set_point(i, y);
        }
    }

    fn draw_vertical_line(&mut self, line: (Point, Point)) {
        let x = line.0.x;
        let min = min(line.0.y, line.1.y);
        let max = max(line.0.y, line.1.y);
        for i in min..=max {
            self.set_point(x, i);
        }
    }

    fn print(self) {
        println!("Content Length: {}", self.contents.len());
        println!();
        for mut _line in 0..self._y_size {
            for j in 0..self._x_size {
                print!(" {} ", self.contents[(j + (_line * self._step)) as usize]);
            }
            println!();
            _line += 1;
        }
    }
}

impl Point {
    pub fn new(s: &str) -> Self {
        let coord_str: Vec<&str> = s.split(',').collect();
        Point {
            x: coord_str[0].parse::<u32>().unwrap(),
            y: coord_str[1].parse::<u32>().unwrap(),
        }
    }
}

fn split_coordinates(s: String) -> (Point, Point) {
    let split = s.split_once(" -> ").unwrap();
    (Point::new(split.0), Point::new(split.1))
}

fn main() -> Result<(), Error> {
    let filename = "../../input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let (mut x_size, mut y_size): (u32, u32) = (0, 0);
    let mut lines: Vec<(Point, Point)> = Vec::new();

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
        // if points.0.x != points.1.x && points.0.y != points.1.y {
        //     continue;
        // }
        lines.push(points);
    }

    let mut map: PointMap = PointMap::new(x_size + 1, y_size + 1);
    for line in lines {
        map.draw_line(line);
    }

    let mut count: u32 = 0;
    for i in map.contents.iter() {
        if i > &1 {
            count += 1;
        }
    }

    // map.print();

    println!("Result: {}", count);

    Ok(())
}
