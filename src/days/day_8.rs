use std::collections::{HashMap, HashSet};

pub fn day_8_1(input: &str) -> u64 {
    let mut instances = 0;
    for line in input.lines() {
        let digits: Vec<&str> = line.split_whitespace().skip(11).take(4).collect();
        instances += digits
            .iter()
            .filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
            .count();
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
        for (&k, &v) in segment_counts.iter() {
            match v {
                4 => char_mapping.insert(k, 'e'),
                6 => char_mapping.insert(k, 'b'),
                9 => char_mapping.insert(k, 'f'),
                _ => None
            };
        }
        // we know e, b, f.
        // now we can deduce c from 1 (len 2), then a from 7 (len 3), d from 4 (len 4), g from 8 (len 7)
        for (sig, p_len) in [('c', 2), ('a', 3), ('d', 4), ('g', 7)] {
            let pattern = patterns.iter().find(|x| x.len() == p_len).unwrap();
            char_mapping.insert(
                pattern
                    .chars()
                    .find(|c| !char_mapping.contains_key(&c))
                    .unwrap(),
                sig,
            );
        }

        // we have the mapping, now need to translate the numbers
        let mut mult = 1000;
        for pattern in &digits {
            sum += translate_num(&char_mapping, pattern) * mult;
            mult = mult / 10;
        }
    }
    sum
}

/// Translates an encoded 7seg pattern into a number based on a mapping of segment signals
fn translate_num(map: &HashMap<char, char>, pattern: &str) -> u64 {
    let mut digit_signals: Vec<HashSet<char>> = Vec::new();
    digit_signals.push(HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']));
    digit_signals.push(HashSet::from(['c', 'f']));
    digit_signals.push(HashSet::from(['a', 'c', 'd', 'e', 'g']));
    digit_signals.push(HashSet::from(['a', 'c', 'd', 'f', 'g']));
    digit_signals.push(HashSet::from(['b', 'c', 'd', 'f']));
    digit_signals.push(HashSet::from(['a', 'b', 'd', 'f', 'g']));
    digit_signals.push(HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']));
    digit_signals.push(HashSet::from(['a', 'c', 'f']));
    digit_signals.push(HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
    digit_signals.push(HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']));

    let mut num_set: HashSet<char> = HashSet::new();
    num_set.extend(pattern.chars().map(|c| map.get(&c).unwrap()));

    return digit_signals.iter().position(|x| *x == num_set).unwrap() as u64;
}
