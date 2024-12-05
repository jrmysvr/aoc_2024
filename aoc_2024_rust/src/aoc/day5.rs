use crate::aoc::input::read_input_for_day;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run() {
    println!("Day 5 Solutions");
    println!("---------------");
    let input = read_input_for_day(5);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = i32;
type Rule = (Num, Num);
type Rules = Vec<Rule>;
type Page = Vec<Num>;
type Pages = Vec<Page>;

fn parse_input(input: &String) -> (Rules, Pages) {
    let rules_str = input.split("\n\n").nth(0).unwrap();
    let pages_str = input.split("\n\n").nth(1).unwrap();

    let rules = rules_str
        .trim()
        .split('\n')
        .map(|line| {
            line.split('|')
                .take(2)
                .map(|s| s.parse::<Num>().unwrap())
                .collect::<Vec<Num>>()
        })
        .map(|rule| (rule[0], rule[1]))
        .collect::<Rules>();

    let pages = pages_str
        .trim()
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<Num>().unwrap())
                .collect::<Page>()
        })
        .collect::<Pages>();

    (rules, pages)
}

type OrderingTable = HashMap<Num, Vec<Num>>;
fn ordering_table_from(rules: &Rules) -> OrderingTable {
    let mut table = OrderingTable::new();
    for (num, lesser) in rules.into_iter() {
        table
            .entry(*num)
            .and_modify(|entry| entry.push(*lesser))
            .or_insert(vec![*lesser]);
    }

    table
}

fn in_correct_order(page: &Page, ordering: &OrderingTable) -> bool {
    /*
    for i in 0..page.len() {
        let num = page[i];
        if page[i + 1..].iter().any(|n| {
            if let Some(lessers) = ordering.get(n) {
                lessers.contains(&num)
            } else {
                false
            }
        }) {
            return false;
        }
    }

    true
    */

    !(0..page.len()).any(|i| {
        page[i + 1..].iter().any(|n| {
            if let Some(lessers) = ordering.get(n) {
                lessers.contains(&page[i])
            } else {
                false
            }
        })
    })
}

fn correct_order_of(page: &Page, ordering: &OrderingTable) -> Page {
    let mut page = page.to_vec();

    page.sort_by(|a, b| {
        if let Some(lessers) = ordering.get(b) {
            if lessers.contains(a) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Equal
        }
    });

    page
}

fn solve_part1(input: &String) -> String {
    let (rules, pages) = parse_input(input);
    let ordering = ordering_table_from(&rules);
    pages
        .iter()
        .filter(|page| in_correct_order(page, &ordering))
        .fold(0, |acc, page| acc + page[page.len() / 2])
        .to_string()
}

fn solve_part2(input: &String) -> String {
    let (rules, pages) = parse_input(input);
    let ordering = ordering_table_from(&rules);
    pages
        .iter()
        .filter(|page| !in_correct_order(page, &ordering))
        .map(|page| correct_order_of(page, &ordering))
        .fold(0, |acc, page| acc + page[page.len() / 2])
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
            "];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let (rules, pages) = parse_input(&get_input(0));
        assert_eq!(rules[0], (47, 53));
        assert_eq!(pages[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_ordering_table() {
        let (rules, _) = parse_input(&get_input(0));
        let ordering = ordering_table_from(&rules);
        assert_eq!(ordering[&53], [29, 13]);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "143");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "123");
    }
}
