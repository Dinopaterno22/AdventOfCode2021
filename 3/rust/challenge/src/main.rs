use std::fs::File;
use std::io::{BufRead, BufReader};

fn _binary_to_int(value: &mut u64, vector: Vec<u32>) {
    for (index, _iter) in vector.iter().enumerate() {
        if _iter == &1 {
            let shift = 1 << index;
            *value |= shift;
        }
    }
}

fn _first_part() {
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
    _binary_to_int(&mut gamma, gamma_vec);
    _binary_to_int(&mut epsilon, epsilon_vec);

    println!("Gamma = {} (Binary: {:012b})", gamma, gamma);
    println!("Epsilon = {} (Binary: {:012b})", epsilon, epsilon);
    println!("{}", gamma * epsilon);
}

fn second_part() {
    let filename = "../../input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let _size: usize = 12;

    let mut _bitcount: Vec<i64> = Vec::new();
    let mut oxygen_word_vec: Vec<String> = Vec::new();
    let mut carbon_word_vec: Vec<String> = Vec::new();
    for _i in 0.._size {
        _bitcount.push(0);
    }
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        oxygen_word_vec.push(line);
    }
    carbon_word_vec.clone_from(&oxygen_word_vec);

    for current_bit in 0.._size {
        if &mut oxygen_word_vec.len() > &mut 1 {
            for i in 0.._size {
                _bitcount[i] = 0;
            }
            for (_index, word) in &mut oxygen_word_vec.iter().enumerate() {
                for (i, c) in word.chars().enumerate() {
                    match c {
                        '1' => {
                            _bitcount[i] += 1;
                        }
                        '0' => {
                            _bitcount[i] -= 1;
                        }
                        _ => println!("Problem"),
                    }
                }
            }
            let bit: char = if _bitcount[current_bit] >= 0 {
                '1'
            } else {
                '0'
            };
            let mut indices: Vec<usize> = Vec::new();
            for (_index, word) in &mut oxygen_word_vec.iter().enumerate() {
                for (i, c) in word.chars().enumerate() {
                    if i == current_bit && c != bit {
                        indices.push(_index);
                    }
                }
            }
            let mut removal: usize = 0;
            for i in indices.iter() {
                oxygen_word_vec.remove(*i - removal);
                removal += 1;
            }
        }
    }
    for current_bit in 0.._size {
        if &mut carbon_word_vec.len() > &mut 1 {
            for i in 0.._size {
                _bitcount[i] = 0;
            }
            for (_index, word) in &mut carbon_word_vec.iter().enumerate() {
                for (i, c) in word.chars().enumerate() {
                    match c {
                        '1' => {
                            _bitcount[i] += 1;
                        }
                        '0' => {
                            _bitcount[i] -= 1;
                        }
                        _ => println!("Problem"),
                    }
                }
            }
            let bit: char = if _bitcount[current_bit] >= 0 {
                '0'
            } else {
                '1'
            };
            let mut indices: Vec<usize> = Vec::new();
            for (_index, word) in &mut carbon_word_vec.iter().enumerate() {
                for (i, c) in word.chars().enumerate() {
                    if i == current_bit && c != bit {
                        indices.push(_index);
                    }
                }
            }
            let mut removal: usize = 0;
            for i in indices.iter() {
                carbon_word_vec.remove(*i - removal);
                removal += 1;
            }
        }
    }

    let mut oxygen_binary_vector: Vec<u32> = Vec::new();
    let mut carbon_binary_vector: Vec<u32> = Vec::new();

    let oxygen_str: String = oxygen_word_vec[0].chars().rev().collect();
    let carbon_str: String = carbon_word_vec[0].chars().rev().collect();

    for (_i, c) in oxygen_str.chars().enumerate() {
        if c == '1' {
            oxygen_binary_vector.push(1);
        } else {
            oxygen_binary_vector.push(0);
        }
    }
    for (_i, c) in carbon_str.chars().enumerate() {
        if c == '1' {
            carbon_binary_vector.push(1);
        } else {
            carbon_binary_vector.push(0);
        }
    }

    let mut oxygen: u64 = 0;
    let mut carbon: u64 = 0;

    _binary_to_int(&mut oxygen, oxygen_binary_vector);
    _binary_to_int(&mut carbon, carbon_binary_vector);
    println!("Oxygen {} (Binary: {:#012b})", oxygen, oxygen);
    println!("Carbon {} (Binary: {:#012b})", carbon, carbon);
    println!("Result {}", oxygen * carbon);
}

fn main() {
    println!("---------------------------------------");
    println!("First Part: ");
    _first_part();
    println!("---------------------------------------");
    println!("Second Part: ");
    second_part();
    println!("---------------------------------------");
}
