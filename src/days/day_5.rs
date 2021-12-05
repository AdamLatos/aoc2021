use std::collections::BTreeMap;

struct Vents(BTreeMap<(u64,u64), i32>);

pub fn day_5_1(input: &str) -> u64 {
    let mut vents = Vents(BTreeMap::new());
    for line in input.lines() {
        let line = line
            .split(&[',', ' ', '-', '>'][..])
            .filter(|&x| !x.is_empty());
        let points: Vec<u64> = line.map(|x| x.parse().unwrap()).collect();
        let (x1, y1, x2, y2) = (points[0], points[1], points[2], points[3]);
        if (x1 == x2) || (y1 == y2) {
            let range_x = if x2 > x1 { x1..=x2 } else { x2..=x1 };
            for i in range_x {
                let range_y = if y2 > y1 { y1..=y2 } else { y2..=y1 };
                for j in range_y {
                    vents.push(i,j);
                }
            }
        }
    }
    vents.count_dangerous()
}

pub fn day_5_2(input: &str) -> u64 {
    let mut vents = Vents(BTreeMap::new());
    for line in input.lines() {
        let line = line
            .split(&[',', ' ', '-', '>'][..])
            .filter(|&x| !x.is_empty());
        let points: Vec<u64> = line.map(|x| x.parse().unwrap()).collect();
        let (x1, y1, x2, y2) = (points[0], points[1], points[2], points[3]);
        if (x1 == x2) || (y1 == y2) {
            let range_x = if x2 > x1 { x1..=x2 } else { x2..=x1 };
            for i in range_x {
                let range_y = if y2 > y1 { y1..=y2 } else { y2..=y1 };
                for j in range_y {
                    vents.push(i,j);
                }
            }
        }
        if abs_diff(x1, x2) == abs_diff(y1, y2) {
            let mut i = x1;
            let mut j = y1;
            for _ in 0..abs_diff(x1, x2) {
                vents.push(i, j);
                i = if x1 < x2 {i+1} else {i-1};
                j = if y1 < y2 {j+1} else {j-1};
            }
            vents.push(x2, y2);
        }
    }
    vents.count_dangerous()
}

impl Vents {
    fn push(&mut self, i: u64, j: u64) {
        if let Some(x) = self.0.get_mut(&(i, j)) {
            *x += 1;
        } else {
            self.0.insert((i, j), 1);
        }
    }

    fn count_dangerous(&self) -> u64 {
        let dangerous_areas = self.0
        .values()
        .filter(|&&x| x > 1)
        .count()
        .try_into()
        .unwrap();
    dangerous_areas
    }
}

fn abs_diff(a: u64, b: u64) -> u64{
    if a > b {
        a - b
    } else {
        b - a
    }
}