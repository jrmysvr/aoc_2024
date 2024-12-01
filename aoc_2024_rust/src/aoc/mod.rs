pub mod input;
pub mod day1;

pub fn run_all() {
    let days = vec![
        day1::run,
    ];

    for day in days {
        day();
    }
}
