day="$1"
if [ -z "$day" ]
then
    echo "Usage: $0 <day number>"
    exit 1
fi

fpath="src/aoc/day$day.rs"

if [ -f "$fpath" ]
then
    echo "File \"$fpath\" already exists!"
    exit 1
fi

echo "Creating a new solution file: $fpath"

cat > "$fpath" <<EOF
use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day $day Solutions");
    println!("---------------");
    let input = read_input_for_day($day);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

fn solve_part1(input: &String) -> String {
    String::new()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = [
        "",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
EOF
