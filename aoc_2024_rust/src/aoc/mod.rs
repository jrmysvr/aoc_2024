pub mod day1;
pub mod day10;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod input;

static DAYS: &[fn()] = &[
    day1::run,
    day2::run,
    day3::run,
    day4::run,
    day5::run,
    day6::run,
    // Day 7
    || {},
    // Day 8
    || {},
    // Day 9
    || {},
    day10::run,
    // Day 11
    || {},
    day12::run,
];

pub fn run_all() {
    for day in DAYS {
        day();
    }
}

pub fn run(day: usize) {
    DAYS[day - 1]()
}
