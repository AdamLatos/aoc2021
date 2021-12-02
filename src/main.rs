use std::env;
use std::fs;

mod days;
use days::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_all(),
        2 => {
            let day = args[1].parse::<usize>().expect("Something went wrong parsing day number");
            run_day(day);
        },
        n => println!("Unexpected number of arguments: {}", n),
    }
}

fn run_day(day: usize) {

    let day_funcs: Vec<fn(&str)->u64> = vec![day_1_1, day_1_2, day_2_1];
    let input = fs::read_to_string(&format!("inputs/day_{:02}.txt", day));
    if input.is_err() {
        return;
    }
    let input = input.unwrap();

    match day {
        1..=25 => {
            if day_funcs.len() >= day*2-1 {
                let ans_1 = day_funcs[(day-1)*2](&input);
                println!("day {}_1:\n{}", day, ans_1);
            }
            if day_funcs.len() >= day*2 {
                let ans_2 = day_funcs[(day-1)*2+1](&input);
                println!("day {}_2:\n{}", day, ans_2);
            }
        },
        _ => println!("Day {} out of bounds", day)
    }
}

fn run_all() {
    println!("running all");
    for day in 1..=25 {
        run_day(day);
    }
}

#[cfg(test)]
mod tests {
    use super::days::*;
    #[test]
    fn test_day_1() {
        let input = "199
            200
            208
            210
            200
            207
            240
            269
            260
            263";
        let ans_1 = day_1_1(input);
        assert_eq!(ans_1, 7);

        let ans_2 = day_1_2(input);
        assert_eq!(ans_2, 5);
    }

    #[test]
    fn test_day_2() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        
        let ans_1 = day_2_1(input);
        assert_eq!(ans_1, 150);
    }
}