use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Board {
    numbers: Vec<Vec<u32>>,
    checked: Vec<Vec<bool>>,
    complete: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            numbers: vec![],
            checked: vec![],
            complete: false,
        }
    }

    fn add_row(&mut self, line: &str) {
        let number_string = line.split_whitespace();
        let num: Vec<u32> = number_string.map(|x| x.parse::<u32>().unwrap()).collect();
        self.checked.push(vec![false; num.len()]);
        self.numbers.push(num);
    }

    fn mark_value(&mut self, num: &u32) -> bool {
        for (_col_idx, col) in self.numbers.iter().enumerate() {
            let row = col.iter().position(|x| x == num);
            if let None = row {
                continue;
            }
            self.checked[_col_idx][row.unwrap()] = true;
        }
        return self.check_win();
    }

    fn check_win(&mut self) -> bool {
        for (_row_idx, row) in self.checked.iter().enumerate() {
            if row.iter().all(|x| *x) {
                self.complete = true;
                return true;
            }
        }

        for i in 0..self.checked.len() {
            if self.checked.iter().map(|x| x[i]).all(|x| x) {
                self.complete = true;
                return true;
            }
        }

        return false;
    }

    fn sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for (check, number) in self.numbers.iter().zip(self.checked.iter()) {
            for (c, n) in number.iter().zip(check.iter()) {
                if !c {
                    sum += n;
                }
            }
        }
        sum
    }

    fn print(&self) {
        for (checked, number) in self.numbers.iter().zip(self.checked.iter()) {
            println!("");
            for (c, n) in number.iter().zip(checked.iter()) {
                print!("{} ({}) ", n, c);
            }
        }
    }
}

fn check_all_boards(boards: Vec<Board>) -> bool {
    for board in boards {
        if !board.complete {
            return false;
        }
    }
    return true;
}

fn first_part() -> Result<(), Error> {
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

    let mut end: bool = false;
    for num in called.iter() {
        for b in boards.iter_mut() {
            if b.mark_value(&num) {
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

fn second_part() -> Result<(), Error> {
    let filename = "../../input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_iterator = reader.lines();
    let mut boards: Vec<Board> = vec![];
    let mut board: Board = Board::new();
    let mut board_set: HashSet<usize> = HashSet::new();
    let mut completed_boards: Vec<usize> = vec![];
    let mut winning_numbers: Vec<u32> = Vec::new();
    let called: Vec<u32> = line_iterator
        .next()
        .unwrap()?
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    line_iterator.next();

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

    for (_index, num) in called.iter().enumerate() {
        for (_board_idx, board) in boards.iter_mut().enumerate() {
            if board_set.contains(&_board_idx) {
                continue;
            }
            if board.mark_value(num) {
                board_set.insert(_board_idx);
                completed_boards.push(_board_idx);
                winning_numbers.push(*num);
            }
        }
    }

    let last_board_idx = *completed_boards.last().unwrap();
    let win = winning_numbers.last().unwrap();
    let sum = boards[last_board_idx].sum();
    let result = win * sum;
    println!("Winning number {}, Sum: {}, Result {}", win, sum, result);
    Ok(())
}

fn main() -> Result<(), Error> {
    first_part()?;
    second_part()?;
    Ok(())
}
