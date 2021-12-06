use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Board {
    numbers: Vec<Vec<u32>>,
    checked: Vec<Vec<bool>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            numbers: vec![],
            checked: vec![],
        }
    }

    fn add_row(&mut self, line: &str) {
        let number_string = line.split_whitespace();
        let num: Vec<u32> = number_string.map(|x| x.parse::<u32>().unwrap()).collect();
        self.checked.push(vec![false; num.len()]);
        self.numbers.push(num);
    }

    fn mark_value(&mut self, num: &u32) {
        for (_vec_index, i) in self.numbers.iter().enumerate() {
            for (_row_index, j) in i.iter().enumerate() {
                if j == num {
                    self.checked[_vec_index][_row_index] = true;
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        let mut col: bool = true;
        let mut line: bool = true;
        // Check lines
        for i in self.checked.iter() {
            line = true;
            for j in i.iter() {
                if j == &false {
                    line = false;
                    break;
                }
            }
            if line {
                break;
            }
        }

        for j in 0..self.checked.len() {
            for c in self.checked.iter() {
                col = true;
                if c[j] == false {
                    col = false;
                    break;
                }
            }
            if col {
                break;
            }
        }
        return line || col;
    }

    fn sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for (_index, i) in self.numbers.iter().enumerate() {
            for (_cenas, j) in i.iter().enumerate() {
                if !self.checked[_index][_cenas] {
                    sum += j;
                }
            }
        }
        sum
    }

    fn print(&self) {
        for (_vec_index, i) in self.numbers.iter().enumerate() {
            println!("");
            for (_row_index, j) in i.iter().enumerate() {
                print!("{} ", j);
            }
            println!("");
            for (_row_index, _j) in i.iter().enumerate() {
                print!("{} ", self.checked[_vec_index][_row_index]);
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let filename = "../../input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_iterator = reader.lines();
    let mut boards: Vec<Board> = vec![];
    let called: Vec<u32> = line_iterator
        .next()
        .unwrap()?
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    line_iterator.next();

    let mut board = Board::new();
    // Populate the board
    for (_index, line) in line_iterator.enumerate() {
        let line = line.unwrap();
        if line == "" {
            // Create a new board
            boards.push(board);
            board = Board::new();
            continue;
        }
        board.add_row(&line);
    }
    boards.push(board);

    // for i in boards.iter() {
    //     i.print();
    //     println!("");
    // }

    // Ate aqui esta tudo bem

    let mut end: bool = false;
    for (_index, num) in called.iter().enumerate() {
        for b in boards.iter_mut() {
            b.mark_value(&num);
            if b.check_win() {
                println!("{}", num * b.sum());
                end = true;
                break;
            }
        }
        if end {
            break;
        }
    }

    Ok(())
}
