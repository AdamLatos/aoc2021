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