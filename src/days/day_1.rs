pub fn day_1_1(input: &str) -> u64 {
    let mut last_num = 0;
    let mut cnt = 0;
    for num in input.split_whitespace().map(|num| num.parse::<u64>().unwrap()) {
        if last_num == 0 {
            last_num = num;
            continue;
        }
        if num > last_num {
            cnt += 1;
        }
        last_num = num;
    }
    return cnt;
}

pub fn day_1_2(input: &str) -> u64 {
    let mut last_triple_sum = 0;
    let mut cnt = 0;
    let input : Vec<u64> = input.split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect();
    for triple in input.windows(3) {
        let triple_sum = triple.iter().sum();
        if last_triple_sum == 0 {
            last_triple_sum = triple_sum;
            continue;
        }
        if triple_sum > last_triple_sum {
            cnt += 1;
        }
        last_triple_sum = triple_sum;
    }
    return cnt;
}