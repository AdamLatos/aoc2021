use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_all(),
        2 => {
            match args[1].parse::<usize>() {
                Ok(day) => run_day(day),
                Err(e) => return Err(Box::new(e)),
            }
        },
        n => println!("Unexpected number of arguments: {}", n),
    }

    Ok(())
}

fn run_day(day: usize) {
    println!("running day {}", day);
}

fn run_all() {
    println!("running all");
}