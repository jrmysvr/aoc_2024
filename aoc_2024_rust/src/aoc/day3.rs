use crate::aoc::input::read_input_for_day;
use fancy_regex::Regex;

pub fn run() {
    println!("Day 3 Solutions");
    println!("---------------");
    let input = read_input_for_day(3);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = i32;
type Pair = (Num, Num);
type Pairs = Vec<Pair>;

fn parse_input(input: &String) -> Pairs {
    let valid_memory_pattern = Regex::new(r"(?<=mul\()([0-9]{1,3}),([0-9]{1,3})(?=\))").unwrap();
    let mut pairs = Pairs::new();
    for capture in valid_memory_pattern.captures_iter(input) {
        let group = capture.unwrap();
        let x = group.get(1).unwrap().as_str();
        let y = group.get(2).unwrap().as_str();
        pairs.push((x.parse::<Num>().unwrap(), y.parse::<Num>().unwrap()));
    }
    pairs
    /*
    valid_memory_pattern
        .captures_iter(input)
        .map(|cap| cap.unwrap())
        .map(|group| {
            (
                group.get(1).unwrap().as_str(),
                group.get(2).unwrap().as_str(),
            )
        })
        .map(|(x, y)| {
            (
                x.parse::<Num>().unwrap(),
                y.parse::<Num>().unwrap(),
            )
        })
        .collect::<Pairs>()
    */
}

fn parse_input_2(input: &String) -> Pairs {
    let valid_memory_pattern =
        Regex::new(r"(don't|do)|(?<=mul\()([0-9]{1,3},[0-9]{1,3})(?=\))").unwrap();
    let mut pairs = Pairs::new();
    let mut do_ = true;
    for capture in valid_memory_pattern.captures_iter(input) {
        let group = capture.unwrap();
        let subcap = group.get(0).unwrap().as_str();
        if subcap == "do" || subcap == "don't" {
            do_ = subcap == "do";
        } else if do_ {
            let nums = subcap
                .split(',')
                .map(|n| n.parse::<Num>().unwrap())
                .collect::<Vec<Num>>();
            pairs.push((nums[0], nums[1]));
        }
    }
    pairs
}

fn solve_part1(input: &String) -> String {
    let pairs = parse_input(&input);
    pairs.iter().fold(0, |acc, (x, y)| acc + x * y).to_string()
}

fn solve_part2(input: &String) -> String {
    let pairs = parse_input_2(&input);
    pairs.iter().fold(0, |acc, (x, y)| acc + x * y).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 2] = [
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let pairs = parse_input(&get_input(0));
        assert_eq!(pairs[0], (2, 4));
    }

    #[test]
    fn test_parse_input_2() {
        let pairs = parse_input_2(&get_input(1));
        assert_eq!(pairs[0], (2, 4));
        assert_eq!(pairs[1], (8, 5));
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "161");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(1)), "48");
    }
}
