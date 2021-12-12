fn main() {
    let input_str = include_str!("./input.txt");

    let mut crab_positions: Vec<i64> = input_str
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut max : i64 = i64::MIN;
    for i in crab_positions.iter(){
        if i > &max {
            max = *i;
        }
    }

    let mut cost_positions :Vec<u64> = Vec::new();

    for i in 0..max {
        let cost : Vec<u64> = crab_positions.iter().map(|x| ((x - i).abs() as u64)).collect();
        cost_positions.push( cost.iter().sum::<u64>());
    }
    let mut min : u64 = u64::MAX;
    for i in cost_positions.iter(){
        if i < &min{
            min = *i;
        }
    }

    println!("Minimum Cost: {}", min);
}
