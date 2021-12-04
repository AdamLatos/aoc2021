pub fn day_4_1(input: &str) -> u64 {
    let mut nums = input.lines().take(1);
    let numbers : Vec<u64> = nums.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();

    println!("Numbers: {:?}", numbers);

    let mut boards : Vec<Bingo> = Vec::new();
    for chunk in input.split_whitespace().skip(1).collect::<Vec<&str>>().chunks(25) {
        println!("Chunk: {:?}", chunk);
        let chunk: [u64; 25] = chunk.iter().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>().try_into().unwrap();
        boards.push(Bingo::fill(chunk));
    }

    for num in numbers {
        for board in boards.iter_mut() {
            board.mark(&num);
            if board.check() {
                return num * board.sum_unmarked();
            }
        }
    }
    0
}

pub fn day_4_2(input: &str) -> u64 {
    0
}

struct Bingo {
    board: [u64; 25],
    marked: [bool; 25],
}

impl Bingo {
    pub fn fill(numbers: [u64; 25]) -> Bingo {
        Bingo {
            board: numbers,
            marked: [false; 25],
        }
    }

    pub fn mark(&mut self, num: &u64) {
        let index = self.board.iter().position(|&n| n==*num);
        match index {
            Some(i) => self.marked[i] = true,
            None => {}, 
        }
    }

    pub fn check(&self) -> bool {
        
        for i in 0..5 {
            if self.marked[i*5..i*5+5].iter().all(|&m| m) {
                return true;
            }
        }
        for i in 0..5 {
            if self.marked[i..].iter().step_by(5).all(|&m| m) {
                return true;
            }
        }
        false
    }

    pub fn sum_unmarked(&self) -> u64 {
        self.board.iter().enumerate().map(|(i, num)| if self.marked[i] == true {0} else {*num}).sum()
    }
}