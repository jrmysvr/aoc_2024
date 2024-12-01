use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_input_for_day(day_number: u8) -> String {
    let input_dir = env::var("AOC_INPUT_DIR").unwrap();
    let day = format!("day{}.txt", day_number);
    let input_day_fpath = Path::new(&input_dir).join(day);
    let file = File::open(input_day_fpath);
    let mut contents = String::new();
    file.expect("Oops... couldn't read this file...")
        .read_to_string(&mut contents)
        .unwrap();
    contents.trim().to_string()
}
