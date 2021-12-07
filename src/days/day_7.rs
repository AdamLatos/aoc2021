pub fn day_7_1(input: &str) -> u64 {
    let input: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let max = *input.iter().max().unwrap();
    let mut best_spot = u64::MAX;
    for i in 1..max {
        let current_spot = fuel_cost(&input, i);
        if current_spot < best_spot {
            best_spot = current_spot;
        }
    }
    best_spot
}

pub fn day_7_2(input: &str) -> u64 {
    let input: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let max = *input.iter().max().unwrap();
    let mut best_spot = u64::MAX;
    for i in 1..max {
        let current_spot = fuel_cost_2(&input, i);
        if current_spot < best_spot {
            best_spot = current_spot;
        }
    }
    best_spot
}

fn fuel_cost(input: &Vec<u64>, position: u64) -> u64 {
    let fuel_cost = input.iter().map(|&x| abs_diff(x, position)).sum();
    fuel_cost
}

fn abs_diff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn fuel_cost_2(input: &Vec<u64>, position: u64) -> u64 {
    let fuel_cost = input
        .iter()
        .map(|&x| abs_diff(x, position))
        .map(|x| (1+x)*(x/2) + (if x%2 != 0 {x/2+1} else {0})  )
        .sum();
    fuel_cost
}
