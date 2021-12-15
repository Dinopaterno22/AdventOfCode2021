use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn first_part() -> Result<(), Error> {
    let filename = "../../input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count: u32 = 0;
    for line in reader.lines() {
        let lines = line?;
        let split: Vec<&str> = lines.split(" | ").collect();
        for i in split[1].split_whitespace() {
            if i.len() == 2 || i.len() == 3 || i.len() == 4 || i.len() == 7 {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);

    Ok(())
}

fn decode(values: Vec<&str>)  -> Vec<HashSet<char>>{
    let one: HashSet<char> = (values.iter().find(|x| x.len() == 2).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    let seven: HashSet<char> = (values.iter().find(|x| x.len() == 3).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    let four: HashSet<char> = (values.iter().find(|x| x.len() == 4).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    let eight: HashSet<char> = (values.iter().find(|x| x.len() == 7).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    let mut two : HashSet<char> = HashSet::new();
    let mut three : HashSet<char> = HashSet::new();
    let mut five : HashSet<char> = HashSet::new();
    let mut six : HashSet<char> = HashSet::new();
    let mut nine : HashSet<char> = HashSet::new();
    let mut zero : HashSet<char> = HashSet::new();

    for i in values.iter(){
        if i.len() == 5 {
            if ((i.chars().collect::<HashSet<char>>()).difference(&seven).collect::<HashSet<&char>>()).len() == 2{
                three = i.chars().collect::<HashSet<char>>();
            }
            else if (i.chars().collect::<HashSet<char>>()).difference(&four).collect::<HashSet<&char>>().len() == 2{
                five = i.chars().collect::<HashSet<char>>();
            }
            else{
                two = i.chars().collect::<HashSet<char>>();
            }
        }
        else if i.len() == 6{
            if((i.chars().collect::<HashSet<char>>()).difference(&four).collect::<HashSet<&char>>()).len() == 2{
                nine = i.chars().collect::<HashSet<char>>();
            }
            else if (i.chars().collect::<HashSet<char>>()).difference(&seven).collect::<HashSet<&char>>().len() == 3{ 
                zero = i.chars().collect::<HashSet<char>>();
            }
            else{
                six = i.chars().collect::<HashSet<char>>();
            }
        }
    }
    let mut numbers : Vec<HashSet<char>> = Vec::new();
    numbers.push(zero);
    numbers.push(one);
    numbers.push(two);
    numbers.push(three);
    numbers.push(four);
    numbers.push(five);
    numbers.push(six);
    numbers.push(seven);
    numbers.push(eight);
    numbers.push(nine);

    return numbers;
}

fn main() -> Result<(), Error> {
    first_part();

    let filename = "../../input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count: u32 = 0;
    for line in reader.lines() {
        let lines = line?;
        let split: Vec<&str> = lines.split(" | ").collect();
        let mut decoded_numbers : Vec<HashSet<char>> = decode(split[0].split_whitespace().collect());
        let mut num : Vec<u32> = Vec::new();
        for  i in split[1].split_whitespace() {
            let num_char : HashSet<char> = i.chars().collect::<HashSet<char>>();
            for (index, j) in decoded_numbers.iter().enumerate() {
                if num_char.difference(&j).collect::<HashSet<_>>().len() == 0 && num_char.len() == j.len(){
                    let digit : u32 = index as u32;
                    // println!("{} ", i);
                    num.push(digit);
                }
            }
            
        }
        count += num[0]*1000 + num[1]*100 + num[2]*10 + num[3];
    }

    println!("Count: {}", count);

    Ok(())
}
