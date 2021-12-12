use std::fs::File;
use std::io::{BufRead, BufReader, Error};


fn main() -> Result<(), Error>{
    let filename = "../../input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count : u32 = 0;
    for line in reader.lines(){
        let lines = line?;
        let split : Vec<&str> = lines.split(" | ").collect();
        for i in split[1].split_whitespace(){
            if i.len() == 2 || i.len() == 3 || i.len() == 4 || i.len() == 7{
                count += 1;
            }
        }
    }

    println!("Count: {}", count);

    Ok(())
}
