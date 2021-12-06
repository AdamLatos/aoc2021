pub fn day_6_1(input: &str) -> u64 {
    // load input
    let mut fish_counts_by_age: [usize; 9] = [0; 9];
    let input: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    for fish in input {
        fish_counts_by_age[fish] += 1;
    }
    // advance simulation
    const DAYS: u64 = 80;

    for _ in 0..DAYS {
        let birthing_fish = fish_counts_by_age[0];
        for i in 1..=8 {
            fish_counts_by_age[i - 1] = fish_counts_by_age[i];
        }
        fish_counts_by_age[6] += birthing_fish;
        fish_counts_by_age[8] = birthing_fish;
    }
    fish_counts_by_age.iter().sum::<usize>() as u64
}

pub fn day_6_2(_input: &str) -> u64 {
    0
}
