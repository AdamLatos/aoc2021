use std::collections::{BTreeSet, HashMap};

pub fn day_15_1(input: &str) -> u64 {
    let mut map = Map::new(input);
    let path = map.a_star();
    map.path_cost(path)
}

pub fn day_15_2(input: &str) -> u64 {
    let mut map = Map::new(input);
    let path = map.a_star_2();
    map.path_cost_2(path)
}

struct Map {
    w_max: usize,
    h_max: usize,
    map: Vec<u8>,

    came_from: HashMap<(usize, usize), (usize, usize)>,
    discovered: BTreeSet<(usize, usize)>,
    g_score: HashMap<(usize, usize), u64>,
    f_score: HashMap<(usize, usize), u64>,
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
            came_from: HashMap::new(),
            discovered: BTreeSet::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
        }
    }

    fn path_cost(&self, path: Vec<(usize, usize)>) -> u64 {
        let mut path = path.iter();
        path.next();
        let mut cost = 0;
        for p in path {
            cost += self.get(*p) as u64;
        }
        cost
    }

    fn path_cost_2(&self, path: Vec<(usize, usize)>) -> u64 {
        let mut path = path.iter();
        path.next();
        let mut cost = 0;
        for p in path {
            cost += self.get_2(*p) as u64;
        }
        cost
    }

    fn avg_dist(&self, a: (usize, usize), b: (usize, usize)) -> u64 {
        let dist = abs_diff(a.0, b.0) + abs_diff(a.1, b.1);
        dist as u64
    }

    fn reconstruct_path(&self, current: (usize, usize)) -> Vec<(usize, usize)> {
        let mut total_path = vec![current];
        let mut current = current;
        while self.came_from.keys().any(|k| *k == current) {
            current = self.came_from[&current];
            total_path.insert(0, current);
        }
        total_path
    }

    fn a_star(&mut self) -> Vec<(usize, usize)> {
        let mut current = (0, 0);
        let end = (self.w_max - 1, self.h_max - 1);
        self.discovered.insert(current);
        self.g_score.insert(current, 0);
        self.f_score.insert(current, self.avg_dist(current, end));

        while !self.discovered.is_empty() {
            current = *self
                .discovered
                .iter()
                .min_by_key(|x| self.f_score.get(x).unwrap())
                .unwrap();
            if current == end {
                return self.reconstruct_path(current);
            }
            self.discovered.remove(&current);
            for n in self.neighbor_points(current) {
                let tentative_g_score = self.g_score[&current] + self.get(n) as u64;
                let neighbor_g_score = self.g_score.get(&n).cloned().unwrap_or(u64::MAX);
                if tentative_g_score < neighbor_g_score {
                    self.came_from.insert(n, current);
                    self.g_score.insert(n, tentative_g_score);
                    self.f_score
                        .insert(n, tentative_g_score + self.avg_dist(n, end));
                    if !self.discovered.contains(&n) {
                        self.discovered.insert(n);
                    }
                }
            }
        }
        vec![current]
    }

    fn a_star_2(&mut self) -> Vec<(usize, usize)> {
        let mut current = (0, 0);
        let end = (self.w_max * 5 - 1, self.h_max * 5 - 1);
        self.discovered.insert(current);
        self.g_score.insert(current, 0);
        self.f_score.insert(current, self.avg_dist(current, end));

        while !self.discovered.is_empty() {
            current = *self
                .discovered
                .iter()
                .min_by_key(|x| self.f_score.get(x).unwrap())
                .unwrap();
            if current == end {
                return self.reconstruct_path(current);
            }
            self.discovered.remove(&current);
            for n in self.neighbor_points_2(current) {
                let weight = self.get_2(n) as usize;
                let tentative_g_score = self.g_score[&current] + weight as u64;
                let neighbor_g_score = self.g_score.get(&n).cloned().unwrap_or(u64::MAX);
                if tentative_g_score < neighbor_g_score {
                    self.came_from.insert(n, current);
                    self.g_score.insert(n, tentative_g_score);
                    self.f_score
                        .insert(n, tentative_g_score + self.avg_dist(n, end));
                    if !self.discovered.contains(&n) {
                        self.discovered.insert(n);
                    }
                }
            }
        }
        vec![current]
    }

    fn get(&self, (w, h): (usize, usize)) -> u8 {
        self.map[w + self.w_max * h]
    }

    fn get_2(&self, (w, h): (usize, usize)) -> usize {
        let w_reduced = w % self.w_max;
        let h_reduced = h % self.h_max;
        let mut weight = self.map[w_reduced + self.w_max * h_reduced] as usize;
        weight += (w / self.w_max + h / self.h_max);
        if weight > 9 {
            weight -= 9;
        }
        weight
    }

    fn neighbor_points(&self, (w, h): (usize, usize)) -> Vec<(usize, usize)> {
        // println!("neighbors of {}, {}:", w, h);
        if w == 0 {
            if h == 0 {
                return vec![(0, 1), (1, 0)];
            } else if h == self.h_max - 1 {
                return vec![(0, h - 1), (1, h)];
            } else {
                return vec![(0, h - 1), (0, h + 1), (1, h)];
            }
        } else if w == self.w_max - 1 {
            if h == 0 {
                return vec![(w, 1), (w - 1, 0)];
            } else if h == self.h_max - 1 {
                return vec![(w, h - 1), (w - 1, h)];
            } else {
                return vec![(w, h - 1), (w, h + 1), (w - 1, h)];
            }
        } else if h == 0 {
            return vec![(w, 1), (w - 1, 0), (w + 1, 0)];
        } else if h == self.h_max - 1 {
            return vec![(w + 1, h), (w - 1, h), (w, h - 1)];
        } else {
            return vec![(w - 1, h), (w + 1, h), (w, h - 1), (w, h + 1)];
        }
    }

    fn neighbor_points_2(&self, (w, h): (usize, usize)) -> Vec<(usize, usize)> {
        // println!("neighbors of {}, {}:", w, h);
        if w == 0 {
            if h == 0 {
                return vec![(0, 1), (1, 0)];
            } else if h == self.h_max * 5 - 1 {
                return vec![(0, h - 1), (1, h)];
            } else {
                return vec![(0, h - 1), (0, h + 1), (1, h)];
            }
        } else if w == self.w_max * 5 - 1 {
            if h == 0 {
                return vec![(w, 1), (w - 1, 0)];
            } else if h == self.h_max * 5 - 1 {
                return vec![(w, h - 1), (w - 1, h)];
            } else {
                return vec![(w, h - 1), (w, h + 1), (w - 1, h)];
            }
        } else if h == 0 {
            return vec![(w, 1), (w - 1, 0), (w + 1, 0)];
        } else if h == self.h_max * 5 - 1 {
            return vec![(w + 1, h), (w - 1, h), (w, h - 1)];
        } else {
            return vec![(w - 1, h), (w + 1, h), (w, h - 1), (w, h + 1)];
        }
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
