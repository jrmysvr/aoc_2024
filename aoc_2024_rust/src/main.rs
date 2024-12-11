mod aoc;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        aoc::run_all()
    } else {
        let day = args[1].parse::<usize>().unwrap();
        aoc::run(day);
    }
}
