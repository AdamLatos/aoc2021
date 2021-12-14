use std::collections::HashSet;

pub fn day_13_1(input: &str) -> u64 {

    let mut instr = Vec::new();
    let mut dots = HashSet::new();

    let mut instr_start = false;
    for line in input.lines() {
        // println!("{}", line);
        if line.trim() == "" {
            instr_start = true;
            continue;
        }
        if instr_start == false {
            let mut vals = line.trim().split(',');
            let (x, y): (usize, usize) = (vals.next().unwrap().parse().unwrap(), vals.next().unwrap().parse().unwrap());
            dots.insert((x, y));
        } else {
            let mut vals = line.trim().split('=');
            let axis = match vals.next().unwrap() {
                "fold along x" => 'x',
                "fold along y" => 'y',
                _ => panic!("woopsie")
            };
            let coord: usize = vals.next().unwrap().parse().unwrap();
            instr.push((axis, coord))
        }
    }
    for (axis, coord) in instr {
        let mut new_points = HashSet::new();
        if axis == 'x' {
            for (x, y) in &dots {
                if *x < coord {
                    continue;
                }
                let new_x = 2 * coord - *x;
                new_points.insert((new_x, *y));
            }
            dots.retain(|(x, _)| *x < coord);
        } else if axis == 'y' {
            for (x, y) in &dots {
                if *y < coord {
                    continue;
                }
                let new_y = 2 * coord - *y;
                new_points.insert((*x, new_y));
            }
            dots.retain(|(_, y)| *y < coord);
        }
        dots.extend(new_points);
        break;
    }
    // for j in 0..=10 {
    //     for i in 0..=10 {
    //         if dots.contains(&(i,j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }
    // println!("{:?}", dots);
    dots.len() as u64
}

pub fn day_13_2(input: &str) -> u64 {

    let mut instr = Vec::new();
    let mut dots = HashSet::new();

    let mut instr_start = false;
    for line in input.lines() {
        // println!("{}", line);
        if line.trim() == "" {
            instr_start = true;
            continue;
        }
        if instr_start == false {
            let mut vals = line.trim().split(',');
            let (x, y): (usize, usize) = (vals.next().unwrap().parse().unwrap(), vals.next().unwrap().parse().unwrap());
            dots.insert((x, y));
        } else {
            let mut vals = line.trim().split('=');
            let axis = match vals.next().unwrap() {
                "fold along x" => 'x',
                "fold along y" => 'y',
                _ => panic!("woopsie")
            };
            let coord: usize = vals.next().unwrap().parse().unwrap();
            instr.push((axis, coord))
        }
    }
    for (axis, coord) in instr {
        let mut new_points = HashSet::new();
        if axis == 'x' {
            for (x, y) in &dots {
                if *x < coord {
                    continue;
                }
                let new_x = 2 * coord - *x;
                new_points.insert((new_x, *y));
            }
            dots.retain(|(x, _)| *x < coord);
        } else if axis == 'y' {
            for (x, y) in &dots {
                if *y < coord {
                    continue;
                }
                let new_y = 2 * coord - *y;
                new_points.insert((*x, new_y));
            }
            dots.retain(|(_, y)| *y < coord);
        }
        dots.extend(new_points);
    }
    for j in 0..=5 {
        for i in 0..=38 {
            if dots.contains(&(i,j)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    0
}