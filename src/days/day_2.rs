pub fn day_2_1(input: &str) -> u64 {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for (cmd, n) in input.split_whitespace().collect::<Vec<&str>>().chunks(2).map(|t| (t[0], t[1].parse::<u64>().unwrap())) {
        
        match cmd {
            "forward" => horizontal_pos += n,
            "up" => depth -= n,
            "down" => depth += n,
            _ => println!("Unexpected command"),
        }
    }
    return horizontal_pos * depth;
}

pub fn day_2_2(input: &str) -> u64 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (cmd, n) in input.split_whitespace().collect::<Vec<&str>>().chunks(2).map(|t| (t[0], t[1].parse::<u64>().unwrap())) {
        
        match cmd {
            "forward" => {
                horizontal_pos += n;
                depth += n * aim;
            }
            "up" => aim -= n,
            "down" => aim += n,
            _ => println!("Unexpected command"),
        }
    }
    return horizontal_pos * depth;
}
