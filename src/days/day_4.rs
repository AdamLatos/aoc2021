pub fn day_4_1(input: &str) -> u64 {
    
    let mut numbers = Vec::new();
    load_numbers(input, &mut numbers);

    let mut boards : Vec<Bingo> = Vec::new();
    load_boards(input, &mut boards);

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
    
    let mut numbers = Vec::new();
    load_numbers(input, &mut numbers);

    let mut boards : Vec<Bingo> = Vec::new();
    load_boards(input, &mut boards);

    for num in numbers {
        for board in boards.iter_mut() {
            board.mark(&num);
        }
        if boards.len() > 1 {
            boards.retain(|board| board.check() == false);
        }
        if boards.len() == 1 && boards[0].check() == true {
            return num * boards[0].sum_unmarked()
        }
    }
    0
}

fn load_numbers(input: &str, numbers: &mut Vec<u64>) {
    let mut nums = input.lines().take(1);
    numbers.extend(nums.next().unwrap().split(',').map(|n| n.parse::<u64>().unwrap()));
}

fn load_boards(input: &str, boards: &mut Vec<Bingo>) {
    for chunk in input.split_whitespace().skip(1).collect::<Vec<&str>>().chunks(25) {
        let chunk: [u64; 25] = chunk.iter().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>().try_into().unwrap();
        boards.push(Bingo::fill(chunk));
    }
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

    pub fn _print(&self) {
        for i in 0..5 {
            for j in 0..5 {
                print!("{} ", self.marked[i+j*5]);
            }
            println!("");
        }
    }
}