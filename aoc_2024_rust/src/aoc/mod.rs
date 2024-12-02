pub mod input;
pub mod day1;
pub mod day2;

pub fn run_all() {
    let days = vec![
        day1::run,
        day2::run,
    ];

    for day in days {
        day();
    }
}
