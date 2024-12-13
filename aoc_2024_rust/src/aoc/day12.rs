use crate::aoc::input::read_input_for_day;
use std::collections::{HashMap, HashSet};

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
type Loc = (usize, usize);
type Locs = Vec<Loc>;
type Num = u32;
type Plot = Loc;
type Region = HashSet<Plot>;
type Regions = Vec<Region>;
type Plots = HashMap<Plant, Regions>;

fn parse_input(input: &String) -> Garden {
    input
        .split('\n')
        .map(|row| row.chars().collect::<Row>())
        .collect::<Garden>()
}

fn find_region_around(plot: &Plot, garden: &Garden) -> Region {
    let mut region = Region::new();
    let mut to_search = vec![(plot.clone(), neighbors_of(plot, garden))];
    region.insert(plot.clone());
    let mut to_search_len = to_search.len();
    while to_search_len > 0 {
        let (plot, neighbors) = to_search.remove(0);
        let plant = garden[plot.0][plot.1];
        for nplot in neighbors {
            let nplant = garden[nplot.0][nplot.1];
            if !region.contains(&nplot) && plant == nplant {
                region.insert(nplot.clone());
                to_search.push((nplot.clone(), neighbors_of(&nplot, garden)));
            }
        }
        to_search_len = to_search.len();
    }

    region
}

fn find_plots_in(garden: &Garden) -> Plots {
    let mut plots = Plots::new();

    let mut searched = Region::new();
    for i in 0..garden.len() {
        for j in 0..garden[0].len() {
            let plant = garden[i][j];
            let plot = (i, j);
            if searched.contains(&plot) {
                continue;
            }

            let region = find_region_around(&plot, garden);
            searched = searched.union(&region).map(|r| *r).collect();

            plots
                .entry(plant)
                .and_modify(|regions| regions.push(region.clone()))
                .or_insert(vec![region]);
        }
    }

    plots
}

fn neighbors_of(plot: &Plot, garden: &Garden) -> Locs {
    let mut neighbors = Locs::new();

    // Up
    if let Some(plot0) = plot.0.checked_sub(1) {
        neighbors.push((plot0, plot.1));
    }
    // Right
    if plot.1 + 1 < garden[0].len() {
        neighbors.push((plot.0, plot.1 + 1));
    }
    // Down
    if plot.0 + 1 < garden.len() {
        neighbors.push((plot.0 + 1, plot.1));
    }
    // Left
    if let Some(plot1) = plot.1.checked_sub(1) {
        neighbors.push((plot.0, plot1));
    }

    neighbors.into_iter().collect::<Locs>()
}

fn calc_area_of(region: &Region) -> Num {
    region.len() as Num
}

fn count_on_border(loc: &Loc, garden: &Garden) -> Num {
    vec![
        loc.0 == 0,
        loc.0 == garden.len() - 1,
        loc.1 == 0,
        loc.1 == garden[0].len() - 1,
    ]
    .into_iter()
    .filter(|l| *l)
    .count() as Num
}

fn calc_perimeter_of(region: &Region, garden: &Garden) -> Num {
    /*
    let mut perimeter = 0;
    for plot in region {
        let plant = garden[plot.0][plot.1];
        for nplot in neighbors_of(plot, garden) {
            let neighbor = garden[nplot.0][nplot.1];
            if plant != neighbor {
                perimeter += 1;
            }
        }
        perimeter += count_on_border(&plot, garden);
    }

    perimeter
    */
    region
        .into_iter()
        .map(|plot| {
            neighbors_of(&plot, garden)
                .into_iter()
                .filter(|nplot| garden[plot.0][plot.1] != garden[nplot.0][nplot.1])
                .count() as Num
                + count_on_border(&plot, garden)
        })
        .fold(0, |acc, count| acc + count)
}

fn solve_part1(input: &String) -> String {
    let garden = parse_input(input);
    find_plots_in(&garden)
        .iter()
        .map(|(_, regions)| {
            regions
                .iter()
                .map(|region| calc_area_of(region) * calc_perimeter_of(region, &garden))
                .fold(0, |acc, prod| acc + prod)
        })
        .fold(0, |acc, sum| acc + sum)
        .to_string()
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
    fn test_find_region_around() {
        let garden = parse_input(&get_input(0));
        let region = find_region_around(&(0, 0), &garden);
        assert_eq!(region, Region::from([(0, 0), (0, 1), (0, 2), (0, 3)]));
    }

    #[test]
    fn test_find_plots() {
        let garden = parse_input(&get_input(0));
        let plots = find_plots_in(&garden);
        assert_eq!(
            plots[&'A'][0],
            Region::from([(0, 0), (0, 1), (0, 2), (0, 3)])
        );

        let garden = parse_input(&get_input(1));
        let plots = find_plots_in(&garden);
        let regions = plots[&'X'].clone();
        assert_eq!(regions[0], Region::from([(1, 1)]));
    }

    #[test]
    fn test_calc_area_of() {
        let garden = parse_input(&get_input(0));
        let plots = find_plots_in(&garden);
        assert_eq!(calc_area_of(&plots[&'A'][0]), 4);
        assert_eq!(calc_area_of(&plots[&'C'][0]), 4);

        let garden = parse_input(&get_input(1));
        let plots = find_plots_in(&garden);
        assert_eq!(calc_area_of(&plots[&'X'][0]), 1);
        assert_eq!(calc_area_of(&plots[&'O'][0]), 21);
    }

    #[test]
    fn test_calc_perimeter_of() {
        let garden = parse_input(&get_input(0));
        let plots = find_plots_in(&garden);
        assert_eq!(calc_perimeter_of(&plots[&'A'][0], &garden), 10);
        assert_eq!(calc_perimeter_of(&plots[&'B'][0], &garden), 8);
        assert_eq!(calc_perimeter_of(&plots[&'C'][0], &garden), 10);
        assert_eq!(calc_perimeter_of(&plots[&'D'][0], &garden), 4);
        assert_eq!(calc_perimeter_of(&plots[&'E'][0], &garden), 8);

        let garden = parse_input(&get_input(1));
        let plots = find_plots_in(&garden);
        assert_eq!(calc_perimeter_of(&plots[&'X'][0], &garden), 4);
        assert_eq!(calc_perimeter_of(&plots[&'O'][0], &garden), 36);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "140");
        assert_eq!(solve_part1(&get_input(1)), "772");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
