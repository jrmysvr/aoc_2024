use crate::aoc::input::read_input_for_day;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    println!("Day 7 Solutions");
    println!("---------------");
    let input = read_input_for_day(7);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = u128;
type Nums = Vec<Num>;
type Calibration = (Num, Nums);
type Calibrations = Vec<Calibration>;

fn line_to_calibration(line: &str) -> Calibration {
    let mut parts = line.split(':');
    let test_val = parts.nth(0).unwrap().parse::<Num>().unwrap();
    let nums = parts
        .nth(0)
        .unwrap()
        .trim()
        .split(' ')
        .map(|part| part.parse::<Num>().unwrap())
        .collect::<Nums>();

    (test_val, nums)
}
fn parse_input(input: &String) -> Calibrations {
    input
        .split('\n')
        .map(line_to_calibration)
        .collect::<Calibrations>()
}

type Op = char;

fn apply(op: Op, a: Num, b: Num) -> Num {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unsupported operation: {op}"),
    }
}

fn operation_results_contains(test_val: Num, nums: &Nums, ops: Vec<char>) -> bool {
    let mut unique_ops = HashSet::<Vec<char>>::new();
    for op_iter in ops.into_iter().combinations_with_replacement(nums.len() - 1) {
        for op_perm in op_iter.clone().into_iter().permutations(op_iter.len()) {
            if !unique_ops.contains(&op_perm) {
                let mut result = nums[0];
                for (num, op) in nums.iter().skip(1).zip(op_perm.clone()) {
                    result = apply(op, result, *num);
                    if result > test_val {
                        break;
                    }
                }
                unique_ops.insert(op_perm.clone());

                if result == test_val {
                    return true;
                }
            }
        }
    }

    false
}

fn solve_part1(input: &String) -> String {
    let calibrations = parse_input(input);
    let mut calibration_result = 0;
    for (test_val, nums) in calibrations {
        calibration_result += if operation_results_contains(test_val, &nums, vec!['*', '+']) {
            test_val
        } else {
            0
        };
    }

    calibration_result.to_string()
}

fn solve_part2(_input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(&get_input(0));
        assert_eq!(parsed[0], (190, vec![10, 19]));
    }

    #[test]
    fn test_operation_results() {
        let parsed = parse_input(&get_input(0));
        let (test_val, nums) = &parsed[0];
        assert!(operation_results_contains(*test_val, &nums, vec!['+', '*']));
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "3749");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
