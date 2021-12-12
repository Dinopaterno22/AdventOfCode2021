use std::io::Error;
use std::collections::VecDeque;

const MORTAL_DAYS: u8 = 80;
const IMMORTAL_DAYS: u16 = 256;

fn mortal_fish() {
    let input_str = include_str!("./input_test.txt");

    let mut fish: Vec<u8> = input_str
        .trim()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    println!("{}", input_str);

    for _day in 0..MORTAL_DAYS {
        let num_new_fish = fish.iter().filter(|x| **x == 0).count();
        let mut new_fish_vec: Vec<u8> = vec![8; num_new_fish];
        fish = fish
            .iter()
            .map(|x| if *x < 1 { 6 } else { *x - 1 })
            .collect();
        fish.append(&mut new_fish_vec);
    }

    println!("Number of Fish: {}", fish.len());
}

fn main() -> Result<(), Error> {
    let input_str = include_str!("./input.txt");

    let mut starting_fish: Vec<u64> = input_str
        .trim()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut fish: VecDeque<u64> = VecDeque::new();

    for i in 0..7{
        fish.push_back(0);
    }

    for i in 0..7 {
        fish[i as usize] = starting_fish.iter().filter(|x| **x == i).count() as u64;
    }

    let mut new_fish: u64 = 0;
    let mut fish_on_8 : u64 = 0;
    let mut fish_on_7 : u64 = 0;

    for i in 0..IMMORTAL_DAYS {
        new_fish = fish[0];
        fish.push_back(fish_on_7 + fish[0]);
        fish.pop_front();
        fish_on_7 = fish_on_8;
        fish_on_8 = new_fish;
    }

    println!("{}", fish.iter().sum::<u64>() + fish_on_7 + fish_on_8);
    Ok(())
}
