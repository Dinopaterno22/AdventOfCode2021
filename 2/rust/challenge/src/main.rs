use std::fs::File;
use std::io::{BufRead, BufReader};

fn depth() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let (mut hor, mut ver): (i32, i32) = (0, 0);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (dir, value) = line.split_once(' ').unwrap();

        match dir {
            "forward" => hor += value.to_string().parse::<i32>().unwrap(),
            "up" => ver -= value.to_string().parse::<i32>().unwrap(),
            "down" => ver += value.to_string().parse::<i32>().unwrap(),
            _ => println!("Nothing happened"),
        }
    }
    println!("Vertical {} Horizontal {}", ver, hor);
    println!("Result: {}", ver * hor);
}

fn aim() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let (mut hor, mut ver, mut aim): (i32, i32, i32) = (0, 0, 0);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (dir, value) = line.split_once(' ').unwrap();

        match dir {
            "forward" => {
                hor += value.to_string().parse::<i32>().unwrap();
                ver += aim * value.to_string().parse::<i32>().unwrap();
            }
            "up" => aim -= value.to_string().parse::<i32>().unwrap(),
            "down" => aim += value.to_string().parse::<i32>().unwrap(),
            _ => println!("Nothing happened"),
        }
    }
    println!("Aim {} Horizontal {} Vertical {}", aim, hor, ver);
    println!("Result: {}", ver * hor);
}

fn main() {
    depth();
    aim();
}
