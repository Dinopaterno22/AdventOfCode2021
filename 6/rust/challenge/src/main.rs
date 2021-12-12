use std::io::Error;

const DAYS: u16 = 256;

fn main() -> Result<(), Error> {
    let input_str = include_str!("./input_test.txt");

    let mut fish: Vec<u8> = input_str
        .trim()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    println!("{}", input_str);

    for _day in 0..DAYS {
        let num_new_fish = fish.iter().filter(|x| **x == 0).count();
        let mut new_fish_vec: Vec<u8> = vec![8; num_new_fish];
        fish = fish.iter()
            .map(|x| if *x < 1 { 6 } else { *x - 1 })
            .collect();
        fish.append(&mut new_fish_vec);
    }

    println!("Number of Fish: {}", fish.len());
    Ok(())
}
