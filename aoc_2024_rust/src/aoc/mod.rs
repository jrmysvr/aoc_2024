pub mod input;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub fn run_all() {
    let days = vec![
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
    ];

    for day in days {
        day();
    }
}
