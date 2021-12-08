use std::collections::{HashMap, HashSet};

pub fn day_8_1(input: &str) -> u64 {
    let mut instances = 0;
    for line in input.lines() {
        let digits: Vec<&str> = line.split_whitespace().skip(11).take(4).collect();
        instances += digits.iter().filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7).count();
    }
    instances as u64
}

// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//  5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// sums:
// a: 8
// b: 6
// c: 8
// d: 7
// e: 4
// f: 9
// g: 7

pub fn day_8_2(input: &str) -> u64 {

    let digit_signals: HashMap<u64, HashSet<char>> = HashMap::from([
        (0, HashSet::from(['a','b','c','e','f','g'])),
        (1, HashSet::from(['c','f'])),
        (2, HashSet::from(['a','c','d','e','g'])),
        (3, HashSet::from(['a','c','d','f','g'])),
        (4, HashSet::from(['b','c','d','f'])),
        (5, HashSet::from(['a','b','d','f','g'])),
        (6, HashSet::from(['a','b','d','e','f','g'])),
        (7, HashSet::from(['a','c','f'])),
        (8, HashSet::from(['a','b','c','d','e','f','g'])),
        (9, HashSet::from(['a','b','c','d','f','g'])),
    ]);

    let mut sum = 0;

    for line in input.lines() {
        let mut char_mapping = HashMap::new();
        let patterns: Vec<&str> = line.split_whitespace().take(10).collect();
        let digits: Vec<&str> = line.split_whitespace().skip(11).take(4).collect();

        // first figure out b, e, f from segment counts
        // count segments
        let mut segment_counts: HashMap<char, u64> = HashMap::new();
        for pattern in &patterns {
            for k in pattern.chars() {
                if let Some(v) = segment_counts.get_mut(&k) {
                    *v += 1;
                } else {
                    segment_counts.insert(k, 1);
                }
            }
        }
        // insert mapping for known elements
        for (&k,&v) in segment_counts.iter() {
            match v {
                4 => {char_mapping.insert(k, 'e');},
                6 => {char_mapping.insert(k, 'b');},
                9 => {char_mapping.insert(k, 'f');},
                _ => {},
            };
        }
        // we know e, b, f.
        // now we can deduce c from 1 (len 2), then a from 7 (len 3), d from 4 (len 4), g from 8 (len 7)
        for pattern in &patterns {
            if pattern.len() != 2 {
                continue;
            }
            let mut c_char = ' ';
            for c in pattern.chars() {
                if !char_mapping.contains_key(&c) {
                    c_char = c;
                    break;
                }
            }
            char_mapping.insert(c_char, 'c');
        }
        for pattern in &patterns {
            if pattern.len() != 3 {
                continue;
            }
            let mut a_char = ' ';
            for c in pattern.chars() {
                if !char_mapping.contains_key(&c) {
                    a_char = c;
                    break;
                }
            }
            char_mapping.insert(a_char, 'a');
        }
        for pattern in &patterns {
            if pattern.len() != 4 {
                continue;
            }
            let mut d_char = ' ';
            for c in pattern.chars() {
                if !char_mapping.contains_key(&c) {
                    d_char = c;
                    break;
                }
            }
            char_mapping.insert(d_char, 'd');
        }
        for pattern in &patterns {
            if pattern.len() != 7 {
                continue;
            }
            let mut g_char = ' ';
            for c in pattern.chars() {
                if !char_mapping.contains_key(&c) {
                    g_char = c;
                    break;
                }
            }
            char_mapping.insert(g_char, 'g');
        }

        // we have the mapping, now need to translate the numbers
        let mut mult = 1000;
        for pattern in &digits {
            let mut num_set: HashSet<char> = HashSet::new();
            // println!("turning {:?}", pattern);
            for c in pattern.chars() {
                num_set.insert(*char_mapping.get(&c).unwrap());
            }
            // println!("into {:?}", num_set);
            for (k, v) in &digit_signals {
                if *v == num_set {
                    // println!("{}", k);
                    sum += k * mult;
                    mult = mult / 10;
                }
            }
        }
    }
    sum
}