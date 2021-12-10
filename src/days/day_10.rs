pub fn day_10_1(input: &str) -> u64 {
    let mut score = 0;
    for line in input.lines() {
        let mut b = Vec::new();
        for c in line.chars() {
            if ingest(&mut b, c) == Err(()) {
                score += get_score(c);
                break;
            }
        }
    }
    score
}


fn ingest(b: &mut Vec<char>, c: char) -> Result<(), ()> {
    match c {
        '(' | '[' | '{' | '<' => {b.push(c); Ok(())}
        ')' => check_bracket(b, '('),
        ']' => check_bracket(b, '['),
        '}' => check_bracket(b, '{'),
        '>' => check_bracket(b, '<'),
        _ => Ok(()),
    }
}

fn check_bracket(b: &mut Vec<char>, expected: char) -> Result<(), ()> {
    if let Some(c) = b.pop() {
        if c == expected {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn get_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn day_10_2(input: &str) -> u64 {
    let mut scores = Vec::new();
    'outer: for line in input.lines() {
        let mut score = 0;
        let mut b = Vec::new();
        for c in line.chars() {
            if ingest(&mut b, c) == Err(()) {
                continue 'outer;
            }
        }
        while let Some(c) = b.pop() {
            score *= 5;
            match c {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => ()
            }
        }
        scores.push(score);
    }
    scores.sort();
    scores[scores.len()/2]
}