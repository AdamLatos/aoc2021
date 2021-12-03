pub fn day_3_1(input: &str) -> u64 {
    
    let len = input.split_whitespace().next().unwrap().len();
    let mut one_counts = vec![0; len];
    let mut zero_counts = vec![0; len];
    
    for num in input.split_whitespace() {
        for (i, bit) in num.chars().enumerate() {
            match bit {
                '0' => zero_counts[i] += 1,
                '1' => one_counts[i] += 1,
                _ => println!("Unexpected char"),
            }
        }
    }
    let mut gamma_str = String::new();
    for i in 0..len {
        if zero_counts[i] > one_counts[i] {
            gamma_str.push('0');            
        } else {
            gamma_str.push('1');
        }
    }
    let gamma = u64::from_str_radix(&gamma_str, 2).unwrap();
    let mask = (1 << len) - 1;
    let epsilon = !gamma & mask;

    return gamma * epsilon;
}

pub fn day_3_2(input: &str) -> u64 {

    // first calculate oxygen generator rating:
    let mut input_vec : Vec<&str> = input.split_whitespace().collect();
    let mut current_bit = 0;

    while input_vec.len() > 1 {
        let res = most_common_in_bit(&input_vec, current_bit);
        let keep_char = match res {
            '0' => '0',
            _ => '1',
        };
        // prune numbers which dont pass the criteria:
        input_vec.retain(|&num| num.as_bytes()[current_bit] as char == keep_char);
        // advance to next bit:
        current_bit += 1;
    }
    let oxygen_generator_rating = u64::from_str_radix(&input_vec[0], 2).unwrap();


    // then similarly calculate CO2 scrubber rating:
    let mut input_vec : Vec<&str> = input.split_whitespace().collect();
    let mut current_bit = 0;

    while input_vec.len() > 1 {
        let res = most_common_in_bit(&input_vec, current_bit);
        let keep_char = match res {
            '0' => '1',
            _ => '0',
        };
        input_vec.retain(|&num| num.as_bytes()[current_bit] as char == keep_char);
        current_bit += 1;
    }
    println!("Last: {}", input_vec[0]);
    let co2_scrubber_rating = u64::from_str_radix(&input_vec[0], 2).unwrap();

    return oxygen_generator_rating * co2_scrubber_rating;
}

fn most_common_in_bit(input: &Vec<&str>, bit: usize) -> char {

    let mut one_counts = 0;
    let mut zero_counts = 0;

    for num in input {
        match num.as_bytes()[bit] as char {
            '0' => zero_counts += 1,
            '1' => one_counts += 1,
            _ => println!("Unexpected char"),
        }
    }
    if zero_counts > one_counts {
        return '0'
    } else if zero_counts < one_counts {
        return '1'
    } else {
        return '='
    }
}