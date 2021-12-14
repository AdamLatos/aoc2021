use std::collections::HashMap;

pub fn day_12_1(input: &str) -> u64 {
    let mut cave_system = Map::new(input);
    let paths = cave_system.find_paths();
    paths.len() as u64
}

#[derive(Debug, PartialEq)]
enum Node {
    Big,
    Small,
    Start,  
    End
}

struct Map {
    edges: HashMap<String, Vec<String>>,
    node_types: HashMap<String, Node>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut edges = HashMap::new();
        let mut node_types = HashMap::new();
        for line in input.lines() {
            let mut nodes = line.split("-");
            let a = nodes.next().unwrap().trim().to_string();
            let b = nodes.next().unwrap().trim().to_string();
            if !edges.contains_key(&a) {
                edges.insert(a.clone(), Vec::new());
                if a == "start".to_owned() {
                    node_types.insert(a.clone(), Node::Start);
                } else if a == "end".to_owned() {
                    node_types.insert(a.clone(), Node::End);
                } else if a == a.clone().to_lowercase() {
                    node_types.insert(a.clone(), Node::Small);
                } else {
                    node_types.insert(a.clone(), Node::Big);
                }
            }
            if !edges.contains_key(&b) {
                edges.insert(b.clone(), Vec::new());
                if b == "start".to_owned() {
                    node_types.insert(b.clone(), Node::Start);
                } else if b == "end".to_owned() {
                    node_types.insert(b.clone(), Node::End);
                } else if b == b.clone().to_lowercase() {
                    node_types.insert(b.clone(), Node::Small);
                } else {
                    node_types.insert(b.clone(), Node::Big);
                }
            }
            edges.get_mut(&a).unwrap().push(b.to_string());
            edges.get_mut(&b).unwrap().push(a.to_string());
        }

        Self {
            edges: edges,
            node_types: node_types,
        }
    }

    fn node_visited(&self, path: &Vec<&str>, node: &str) -> bool {
        if path.iter().any(|&x| x == node) {
            return true
        }
        false
    }

    fn find_paths(&mut self) -> Vec<Vec<&str>> {
        let mut paths = Vec::new();
        let mut current_paths = Vec::new();
        current_paths.push(vec!["start"]);
        loop {
            let paths_left = current_paths.len();
            let mut new_paths = Vec::new();
            if paths_left == 0 {
                break;
            }
            for path in current_paths {
                let last_node = path.last().unwrap();
                for next in self.edges.get(&last_node as &str).unwrap() {
                    if (self.node_visited(&path, next) == true && self.node_types[next] == Node::Small) || self.node_types[next] == Node::Start {
                        continue;
                    } else if self.node_types[next] == Node::End {
                        let mut end_path = path.clone();
                        end_path.push(next);
                        paths.push(end_path);
                    } else {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        new_paths.push(new_path);
                    }
                }
            }
            current_paths = new_paths;
        }
        paths
    }

    fn find_paths_dup(&mut self) -> Vec<Vec<&str>> {
        let mut paths = Vec::new();

        let mut current_paths = Vec::new();
        current_paths.push((vec!["start"], false));
        loop {
            let paths_left = current_paths.len();
            let mut new_paths = Vec::new();
            if paths_left == 0 {
                break;
            }
            for path in current_paths {
                let last_node = path.0.last().unwrap();
                let has_duplicate = path.1;
                // println!("last node: {}", last_node);
                for next in self.edges.get(&last_node as &str).unwrap() {
                    if self.node_types[next] == Node::Start {
                        continue;
                    } else if self.node_visited(&(path.0), next) == true && self.node_types[next] == Node::Small {
                        if has_duplicate == true {
                            continue;
                        } else {
                            let mut new_path = path.0.clone();
                            new_path.push(next);
                            new_paths.push((new_path, true));
                        }
                    } else if self.node_types[next] == Node::End {
                        let mut end_path = path.0.clone();
                        end_path.push(next);
                        paths.push(end_path);
                    } else {
                        let mut new_path = path.0.clone();
                        new_path.push(next);
                        new_paths.push((new_path, has_duplicate));
                    }
                }
            }
            current_paths = new_paths;
        }
        paths
    }
}

pub fn day_12_2(input: &str) -> u64 {
    let mut cave_system = Map::new(input);
    let paths = cave_system.find_paths_dup();
    // println!("found paths: {:?}", paths);
    paths.len() as u64
}