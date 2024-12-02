use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 2 Solutions");
    println!("---------------");
    let input = read_input_for_day(2);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

fn solve_part1(input: &String) -> String {
    let reports = parse_input(input);
    reports
        .iter()
        .map(|report| if is_safe(report) { 1 } else { 0 })
        .sum::<Num>()
        .to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

type Num = i32;
type Report = Vec<Num>;
type Reports = Vec<Report>;

fn parse_input(input: &String) -> Reports {
    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<Num>().unwrap())
                .collect::<Report>()
        })
        .collect::<Reports>()
}

fn all_decreasing(report: &Report) -> bool {
    report
        .windows(2)
        .filter(|levels| levels[0] > levels[1])
        .count()
        == report.len() - 1
}

fn all_increasing(report: &Report) -> bool {
    report
        .windows(2)
        .filter(|levels| levels[0] < levels[1])
        .count()
        == report.len() - 1
}

fn all_respect_max_diff(report: &Report, max_diff: Num) -> bool {
    report
        .windows(2)
        .filter(|levels| Num::abs(levels[0] - levels[1]) <= max_diff)
        .count()
        == report.len() - 1
}

fn is_safe(report: &Report) -> bool {
    (all_decreasing(report) || all_increasing(report)) && all_respect_max_diff(report, 3)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(&get_input(0));
        assert_eq!(parsed[0], vec![7, 6, 4, 2, 1]);
    }

    #[test]
    fn test_all_decreasing() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(all_decreasing(&report));

        let report = vec![7, 6, 4, 2, 4];
        assert!(!all_decreasing(&report));
    }

    #[test]
    fn test_all_increasing() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(all_increasing(&report));

        let report = vec![1, 3, 6, 9, 7];
        assert!(!all_increasing(&report));
    }

    #[test]
    fn test_all_respect_max_diff() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(all_respect_max_diff(&report, 3));

        let report = vec![1, 3, 6, 5, 9];
        assert!(!all_respect_max_diff(&report, 3));
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "2");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
