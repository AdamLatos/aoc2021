pub fn day_11_1(input: &str) -> u64 {
    let mut octopi = Octopi::new(input);
    let steps = 100;
    let mut total_flashes = 0;
    for _ in 1..=steps {
        total_flashes += octopi.advance_step();
    }
    total_flashes as u64
}

struct Octopi {
    w_max: usize,
    h_max: usize,
    map: Vec<u8>,
}

impl Octopi {

    fn new(input: &str) -> Self {
        Self {
            w_max: input.lines().next().unwrap().len(),
            h_max: input.lines().count(),
            map: input
                .chars()
                .filter(|x| !x.is_whitespace())
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }

    fn advance_step(&mut self) -> usize {

        let mut flashed = vec![false; self.w_max*self.h_max];

        for octopus in &mut self.map {
            *octopus += 1;
        }
        loop {
            let mut any_flashed = 0;
            for w in 0..self.w_max {
                for h in 0..self.h_max {
                    if self.get(w, h) > 9 && !flashed[w+self.w_max*h] {
                        any_flashed = 1;
                        flashed[w+self.w_max*h] = true;
                        let neighs = self.neighbors(w, h);
                        for (nw, nh) in neighs {
                            if nw==0 && nh==6 {
                            }
                            self.map[nw + self.w_max * nh] += 1;
                        }
                    }
                }
            }

            if any_flashed == 0 {
                for oc in &mut self.map {if *oc > 9 {*oc = 0}}
                break;
            }
        }
        flashed.iter().filter(|&&x| x==true).count()
    }

    fn get(&self, w: usize, h: usize) -> u8 {
        self.map[w + self.w_max * h]
    }

    fn _print(&self) {
        for h in 0..self.h_max {
            for w in 0..self.w_max {
                print!("{}", self.get(w, h));
            }
            println!("");
        }
        println!("");
    }

    fn neighbors(&self, w: usize, h: usize) -> Vec<(usize, usize)> {
        let mut neighs = Vec::new();
        for wi in [-1,0,1] {
            for hi in [-1,0,1] {
                if (wi == 0) && (hi == 0) {
                    continue;
                }
                if let Ok(w_index) = usize::try_from(w as i64 + wi) {
                    if w_index >= self.w_max {
                        continue;
                    }
                    if let Ok(h_index) = usize::try_from(h as i64 + hi) {
                        if h_index >= self.h_max {
                            continue;
                        }
                        if let Some(_) = self.map.get(w_index + self.w_max * h_index) {
                            neighs.push((w_index, h_index));
                        }
                    }
                }
            }
        }
        neighs
    }
}


pub fn day_11_2(input: &str) -> u64 {
    let mut octopi = Octopi::new(input);
    let steps = 100000;
    let max_flashes = octopi.w_max * octopi.h_max;
    for s in 1..=steps {
        let flashes = octopi.advance_step();
        if flashes == max_flashes {
            return s as u64
        }
    }
    0
}