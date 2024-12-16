use crate::aoc::input::read_input_for_day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("Day 8 Solutions");
    println!("---------------");
    let input = read_input_for_day(8);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Loc = (isize, isize);
type Locs = Vec<Loc>;
type Node = char;
type Antenna = Node;
type Antennas = HashMap<Antenna, Locs>;
type Map = Vec<Vec<Antenna>>;
fn parse_input(input: &String) -> Map {
    input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<Antenna>>())
        .collect::<Map>()
}

fn find_antennas_in(map: &Map) -> Antennas {
    let mut antennas = HashMap::<Antenna, Locs>::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let a = map[i][j];
            if a != '.' && a != '#' {
                let loc = (i as isize, j as isize);
                antennas
                    .entry(a)
                    .and_modify(|locs| locs.push(loc.clone()))
                    .or_insert(vec![loc.clone()]);
            }
        }
    }

    antennas
}

fn in_bounds(loc: &Loc, map: &Map) -> bool {
    loc.0 >= 0 && loc.0 < map.len() as isize && loc.1 >= 0 && loc.1 < map[0].len() as isize
}

fn calc_two_antinodes_of(loc_a: &Loc, loc_b: &Loc, map: &Map) -> (Option<Loc>, Option<Loc>) {
    let diff_0 = loc_b.0 - loc_a.0;
    let diff_1 = loc_b.1 - loc_a.1;
    let loc_a = (loc_a.0 - diff_0, loc_a.1 - diff_1);
    let loc_b = (loc_b.0 + diff_0, loc_b.1 + diff_1);

    (
        if in_bounds(&loc_a, &map) {
            Some(loc_a)
        } else {
            None
        },
        if in_bounds(&loc_b, &map) {
            Some(loc_b)
        } else {
            None
        },
    )
}

fn calc_antinodes_inline_with(loc_a: &Loc, loc_b: &Loc, map: &Map) -> Locs {
    let mut antinodes = Locs::new();
    let diff_0 = loc_b.0 - loc_a.0;
    let diff_1 = loc_b.1 - loc_a.1;
    let mut i = 1;
    loop {
        let loc = (loc_b.0 + (diff_0 * i), loc_b.1 + (diff_1 * i));
        i += 1;

        if in_bounds(&loc, &map) {
            antinodes.push(loc);
        } else {
            break;
        };
    }

    antinodes
}

fn calc_antinodes_of(locs: &Locs, map: &Map) -> Locs {
    let mut antinode_locs = Locs::new();
    for ab in locs.into_iter().combinations(2) {
        let (opt_a, opt_b) = calc_two_antinodes_of(&ab[0], &ab[1], map);
        if let Some(loc_a) = opt_a {
            antinode_locs.push(loc_a);
        }
        if let Some(loc_b) = opt_b {
            antinode_locs.push(loc_b);
        }
    }

    antinode_locs
}

fn calc_antinodes_2_of(locs: &Locs, map: &Map) -> Locs {
    let mut antinode_locs = Locs::new();
    for ab in locs.into_iter().combinations(2) {
        antinode_locs.extend(calc_antinodes_inline_with(&ab[0], &ab[1], map));
        antinode_locs.extend(calc_antinodes_inline_with(&ab[1], &ab[0], map));
        antinode_locs.push(ab[0].clone());
        antinode_locs.push(ab[1].clone());
    }

    antinode_locs
}

fn calc_all_antinodes_of(antennas: &Antennas, map: &Map) -> Locs {
    antennas
        .iter()
        .map(|(_, locs)| calc_antinodes_of(locs, map))
        .flatten()
        .collect::<Locs>()
}

fn calc_all_antinodes_2_of(antennas: &Antennas, map: &Map) -> Locs {
    antennas
        .iter()
        .map(|(_, locs)| calc_antinodes_2_of(locs, map))
        .flatten()
        .collect::<Locs>()
}

fn solve_part1(input: &String) -> String {
    let map = parse_input(input);
    let antennas = find_antennas_in(&map);
    let antinodes = calc_all_antinodes_of(&antennas, &map);
    HashSet::<Loc>::from_iter(antinodes.into_iter())
        .len()
        .to_string()
}

fn solve_part2(input: &String) -> String {
    let map = parse_input(input);
    let antennas = find_antennas_in(&map);
    let antinodes = calc_all_antinodes_2_of(&antennas, &map);

    HashSet::<Loc>::from_iter(antinodes.into_iter())
        .len()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &[
        "
..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
           ",
        "
..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........
    ",
        "
..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........
",
        "
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
",
        "
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let map = parse_input(&get_input(0));
        assert_eq!(map[0][0], '.');
        assert_eq!(map[3][4], 'a');
    }

    #[test]
    fn test_calc_all_antinodes_of() {
        let map = parse_input(&get_input(0));
        let antennas = find_antennas_in(&map);
        let antinodes = calc_all_antinodes_of(&antennas, &map);
        assert!(antinodes.contains(&(1, 3)));
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "2");
        assert_eq!(solve_part1(&get_input(1)), "4");
        assert_eq!(solve_part1(&get_input(2)), "4");
        assert_eq!(solve_part1(&get_input(3)), "14");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(4)), "9");
        assert_eq!(solve_part2(&get_input(3)), "34");
    }
}
