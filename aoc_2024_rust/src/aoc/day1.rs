use crate::aoc::input::read_input_for_day;
use std::collections::HashMap;

pub fn run() {
    println!("Day 1 Solutions");
    println!("---------------");
    let input = read_input_for_day(1);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

fn clean_input(input: &str) -> String {
    String::from(input)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

type Num = i32;

fn parse_input(input: &String) -> (Vec<Num>, Vec<Num>) {
    let nums = clean_input(input)
        .split(" ")
        .map(|s| String::from(s).parse::<Num>().unwrap())
        .collect::<Vec<Num>>();

    let mut a = Vec::new();
    let mut b = Vec::new();
    for i in 0..nums.len() {
        if i % 2 == 0 {
            a.push(nums[i]);
        } else {
            b.push(nums[i]);
        }
    }

    (a, b)
}

fn solve_part1(input: &String) -> Num {
    let (mut a, mut b) = parse_input(&input);

    // Find sum of "distances" between sorted list elements
    a.sort();
    b.sort();
    a.into_iter()
        .zip(b.into_iter())
        .fold(0, |acc, (ai, bi)| acc + (ai - bi).abs())
}

fn solve_part2(input: &String) -> Num {
    let (a, b) = parse_input(&input);

    let mut bmap = HashMap::<Num, Num>::new();
    for bi in b.into_iter() {
        bmap.entry(bi).and_modify(|x| *x += 1).or_insert(1);
    }

    // Find the similarity score between lists
    a.into_iter().fold(0, |similarity, ai| {
        similarity + ai * bmap.get(&ai).unwrap_or(&0)
    })
}

#[cfg(test)]
mod test_part_1 {
    use crate::aoc::day1::*;

    const INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    fn get_input() -> String {
        String::from(INPUT)
    }

    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(&get_input());
        assert_eq!(parsed_input.0, vec![3, 4, 2, 1, 3, 3]);
    }

    #[test]
    fn test_full() {
        assert_eq!(11, solve_part1(&get_input()));
    }
}

#[cfg(test)]
mod test_part_2 {
    use crate::aoc::day1::*;

    const INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    fn get_input() -> String {
        String::from(INPUT)
    }

    #[test]
    fn test_full() {
        assert_eq!(31, solve_part2(&get_input()));
    }
}
