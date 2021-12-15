use std::collections::HashMap;

pub fn day_14_1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut polymer: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut lut = HashMap::new();
    for line in lines {
        let mut toks = line.trim().split_whitespace();
        let pattern = toks.next().unwrap();
        toks.next();
        let product = toks.next().unwrap().chars().nth(0).unwrap();
        // println!("Pattern: {} Product: {}", pattern, product);
        lut.insert((pattern.chars().nth(0).unwrap(), pattern.chars().nth(1).unwrap()), product);
    }

    let mut counts = HashMap::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            counts.insert(c, 0);
        }
    }

    // println!("Polymer: {:?}", polymer);
    
    let steps = 10;

    for _ in 1..=steps {
        let mut inserts = Vec::new();
        // first find the new polymers
        for c in polymer.windows(2) {
            let a = c[0];
            let b = c[1];
            let product = lut.get(&(a,b)).unwrap();
            inserts.push(product);
        }
        // then insert them in the old one
        let last = polymer[polymer.len() - 1];
        let interleaved = polymer.iter().zip(inserts).flat_map(|x| [*x.0, *x.1]).collect();
        polymer = interleaved;
        polymer.push(last);
        // println!("After step 1: {:?}", polymer);
    }

    let keys: Vec<char> = counts.keys().map(|x| *x).collect();
    for k in keys {
        let count = counts.get_mut(&k).unwrap();
        *count = polymer.iter().filter(|x| **x==k).count();
    }
    // println!("{:?}", counts);
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    let res = (*max as i64 - *min as i64) as u64;
    res
}

pub fn day_14_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let polymer: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut pairs = HashMap::new();
    let mut lut = HashMap::new();
    for line in lines {
        let mut toks = line.trim().split_whitespace();
        let pattern = toks.next().unwrap();
        toks.next();
        let product = toks.next().unwrap().chars().nth(0).unwrap();
        // println!("Pattern: {} Product: {}", pattern, product);
        lut.insert((pattern.chars().nth(0).unwrap(), pattern.chars().nth(1).unwrap()), product);
        pairs.insert((pattern.chars().nth(0).unwrap(), pattern.chars().nth(1).unwrap()), 0);
    }

    for c in polymer.windows(2) {
        let a = c[0];
        let b = c[1];
        let pair = pairs.get_mut(&(a,b)).unwrap();
        *pair += 1;
    }

    let mut counts: HashMap<char, u64> = HashMap::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            counts.insert(c, 0);
        }
    }

    for c in polymer {
        let cnt = counts.get_mut(&c).unwrap();
        *cnt += 1;
    }

    let steps = 40;

    for _s in 1..=steps {
        // first find the new polymers
        let mut reactions = pairs.clone();
        for (k, &mut v) in reactions.iter_mut() {
            let prod = lut.get(k).unwrap();
            let old_pair = pairs.get_mut(k).unwrap();
            *old_pair -= v;
            let first_new = ((*k).0, *prod);
            let second_new = (*prod, (*k).1);
            let first = pairs.get_mut(&first_new).unwrap();
            *first += v;
            let second = pairs.get_mut(&second_new).unwrap();
            *second += v;
            let cnt = counts.get_mut(prod).unwrap();
            *cnt += v;
        }
        // println!("After step {}: {:?}", _s, pairs);
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    let res = (*max as i64 - *min as i64) as u64;
    res
}