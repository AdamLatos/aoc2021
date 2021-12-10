use std::collections::HashSet;

pub fn day_9_1(input: &str) -> u64 {
    let m = Map::new(input);
    m.sum_low_point()
}

struct Map {
    w_max: usize,
    h_max: usize,
    map: Vec<u8>,
}

impl Map {

    fn new(input: &str) -> Self {
        Map {
            w_max: input.lines().next().unwrap().len(),
            h_max: input.lines().count(),
            map: input
                .chars()
                .filter(|x| !x.is_whitespace())
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }

    fn get(&self, w: usize, h: usize) -> u8 {
        // println!("getting {}, {}", w, h);
        self.map[w + self.w_max * h]
    }

    fn neighbors(&self, w: usize, h: usize) -> Vec<u8> {
        // println!("neighbors of {}, {}:", w, h);
        if w == 0 {
            if h == 0 {
                return vec![self.get(0,1), self.get(1,0)]
            } else if h == self.h_max - 1 {
                return vec![self.get(0,h-1), self.get(1,h)]
            } else {
                return vec![self.get(0,h-1), self.get(0,h+1), self.get(1,h)]
            }
        } else if w == self.w_max - 1 {
            if h == 0 {
                return vec![self.get(w,1), self.get(w-1,0)]
            } else if h == self.h_max - 1 {
                return vec![self.get(w,h-1), self.get(w-1,h)]
            } else {
                return vec![self.get(w,h-1), self.get(w,h+1), self.get(w-1,h)]
            }
        } else if h == 0 {
            return vec![self.get(w,1), self.get(w-1,0), self.get(w+1,0)]
        } else if h == self.h_max - 1 {
            return vec![self.get(w+1,h), self.get(w-1,h), self.get(w,h-1)]
        } else {
            return vec![self.get(w-1,h), self.get(w+1,h), self.get(w,h-1), self.get(w,h+1)]
        } 
    }

    fn neighbor_points(&self, (w, h): (usize, usize)) -> Vec<(usize, usize)> {
        // println!("neighbors of {}, {}:", w, h);
        if w == 0 {
            if h == 0 {
                return vec![(0,1), (1,0)]
            } else if h == self.h_max - 1 {
                return vec![(0,h-1), (1,h)]
            } else {
                return vec![(0,h-1), (0,h+1), (1,h)]
            }
        } else if w == self.w_max - 1 {
            if h == 0 {
                return vec![(w,1), (w-1,0)]
            } else if h == self.h_max - 1 {
                return vec![(w,h-1), (w-1,h)]
            } else {
                return vec![(w,h-1), (w,h+1), (w-1,h)]
            }
        } else if h == 0 {
            return vec![(w,1), (w-1,0), (w+1,0)]
        } else if h == self.h_max - 1 {
            return vec![(w+1,h), (w-1,h), (w,h-1)]
        } else {
            return vec![(w-1,h), (w+1,h), (w,h-1), (w,h+1)]
        } 
    }

    fn sum_low_point(&self) -> u64 {
        let mut sum: u64 = 0;
        for w in 0..self.w_max {
            for h in 0..self.h_max {
                let p = self.get(w, h);
                let neighbors = self.neighbors(w, h);
                if neighbors.iter().all(|&x| p < x) {
                    sum += (p + 1) as u64;
                }
            }
        }
        sum
    }

    fn find_basin_size(&self, w: usize, h: usize) -> u64 {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert((w, h));
        let mut neighbors = vec![(w, h)];
        while (&neighbors).len() > 0 {
            let mut new_neighbors = Vec::new();
            for n in &neighbors {
                let mut ns = self.neighbor_points(*n);
                ns.retain(|&(w,h)| self.get(w, h) > self.get(n.0, n.1) && self.get(w, h) != 9);
                new_neighbors.append(&mut ns);
            }
            // println!("New neighbors: {:?}", new_neighbors);
            basin.extend(neighbors.clone());
            neighbors = new_neighbors;
            
        }
        // println!("basin: {:?}", basin);
        basin.iter().count() as u64
    }

    fn find_3_largest_basins(&self) -> u64 {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        for w in 0..self.w_max {
            for h in 0..self.h_max {
                let p = self.get(w, h);
                let neighbors = self.neighbors(w, h);
                if neighbors.iter().all(|&x| p < x) {
                    let basin_size = self.find_basin_size(w, h);
                    if basin_size > first {
                        third = second;
                        second = first;
                        first = basin_size;
                    } else if basin_size > second {
                        third = second;
                        second = basin_size;
                    } else if basin_size > third {
                        third = basin_size;
                    }
                    // println!("basin size: {} ({},{})", basin_size, w, h);
                }
            }
        }
        first * second * third
    }
}

pub fn day_9_2(input: &str) -> u64 {
    let m = Map::new(input);
    m.find_3_largest_basins()
}
