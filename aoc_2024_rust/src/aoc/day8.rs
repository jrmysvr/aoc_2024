use crate::aoc::input::read_input_for_day;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run() {
    println!("Day 8 Solutions");
    println!("---------------");
    let input = read_input_for_day(8);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Loc = (usize, usize);
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
                antennas
                    .entry(a)
                    .and_modify(|locs| locs.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        }
    }

    antennas
}

fn calc_antinodes_of(antennas: &Antennas, map: &Map) -> Locs {
    let mut antinode_locs = Locs::new();

    for (antenna, locs) in antennas {
        for (a, b) in locs.into_iter().tuple_windows() {
            if let Some(diff_0) = b.0.checked_sub(a.0) {
                if let Some(diff_1) = b.1.checked_sub(a.1) {
                    if diff_0 < map.len() && diff_1 < map[0].len() {
                        continue;
                    }
                }
            }
        }
    }

    antinode_locs
}

fn solve_part1(input: &String) -> String {
    let map = parse_input(input);
    let antennas = find_antennas_in(&map);
    println!("{antennas:?}");
    let antinodes = calc_antinodes_of(&antennas, &map);
    String::new()
}

fn solve_part2(input: &String) -> String {
    String::new()
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
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "14");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
