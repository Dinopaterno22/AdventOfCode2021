use std::fs::File;
use std::io::{BufRead, BufReader};

fn binary_to_int(value: &mut u64, vector: Vec<u32>) {
    for (index, _iter) in vector.iter().enumerate() {
        if _iter == &1 {
            let shift = 1 << index;
            *value |= shift;
        }
    }
}

fn main() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let _size: usize = 12;
    let mut _bitcount: Vec<i64> = Vec::new();
    let mut gamma_vec: Vec<u32> = Vec::new();
    let mut epsilon_vec: Vec<u32> = Vec::new();
    let (mut gamma, mut epsilon): (u64, u64) = (0, 0);

    for _i in 0.._size {
        _bitcount.push(0);
    }

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => {
                    _bitcount[_size - 1 - i] += 1;
                }
                '0' => {
                    _bitcount[_size - 1 - i] -= 1;
                }
                _ => println!("Problem"),
            }
        }
    }

    for _iter in _bitcount.iter() {
        if _iter > &0 {
            gamma_vec.push(1);
            epsilon_vec.push(0);
        } else {
            gamma_vec.push(0);
            epsilon_vec.push(1);
        }
    }
    binary_to_int(&mut gamma, gamma_vec);
    binary_to_int(&mut epsilon, epsilon_vec);

    println!("Gamma = {} (Binary: {:012b})", gamma, gamma);
    println!("Epsilon = {} (Binary: {:012b})", epsilon, epsilon);
    println!("{}", gamma * epsilon);
}
