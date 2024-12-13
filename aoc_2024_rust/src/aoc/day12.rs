use crate::aoc::input::read_input_for_day;
use std::collections::HashMap;

pub fn run() {
    println!("Day 12 Solutions");
    println!("---------------");
    let input = read_input_for_day(12);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Plant = char;
type Row = Vec<Plant>;
type Garden = Vec<Row>;
fn parse_input(input: &String) -> Garden {
    input
        .split('\n')
        .map(|row| row.chars().collect::<Row>())
        .collect::<Garden>()
}

type Loc = (usize, usize);
type Locs = Vec<Loc>;
type Plots = HashMap<Plant, Locs>;
fn find_plots_in(garden: &Garden) -> Plots {
    let mut plots = Plots::new();

    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            let plant = garden[i][j];
            let loc = (i, j);
            plots.entry(plant).and_modify(|locs| locs.push(loc)).or_insert(vec![loc]);
        }
    }

    plots
}

type Num = u32;
type Plot = Locs;
fn calc_area_of(plot: &Plot) -> Num {
    0
}
fn solve_part1(input: &String) -> String {
    let garden = parse_input(input);
    let plots = find_plots_in(&garden);

    String::new()
}

fn solve_part2(_input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &[
        "
AAAA
BBCD
BBCC
EEEC",
        "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let garden = parse_input(&get_input(0));
        assert_eq!(garden[0][0], 'A');
        assert_eq!(garden[3][3], 'C');
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "");
    }

    #[test]
    fn test_find_plots() {
        let garden = parse_input(&get_input(0));
        let plots = find_plots_in(&garden);
        assert_eq!(plots[&'A'], vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
    }

    #[test]
    fn test_calc_area_of() {
        let garden = parse_input(&get_input(0));
        let plots = find_plots_in(&garden);
        assert_eq!(calc_area_of(&plots[&'A']), 4);
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
