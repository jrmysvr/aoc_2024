mod aoc;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 1 {
        let day = &args[1].parse::<usize>().unwrap();
        aoc::run(*day);
    } else {
        aoc::run_all()
    }
}
