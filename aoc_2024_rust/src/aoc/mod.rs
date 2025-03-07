pub mod day1;
pub mod day10;
pub mod day11;
pub mod day14;
pub mod day15;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod input;

static DAYS: &[fn()] = &[
    day1::run,
    day2::run,
    day3::run,
    day4::run,
    day5::run,
    day6::run,
    day7::run,
    day8::run,
    // Day 9
    || {},
    day10::run,
    day11::run,
    // Day 12
    || {},
    // Day 13
    || {},
    day14::run,
    day15::run,
];

pub fn run_all() {
    for day in DAYS {
        day();
    }
}

pub fn run(day: usize) {
    DAYS[day - 1]()
}
