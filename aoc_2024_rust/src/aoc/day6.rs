use crate::aoc::input::read_input_for_day;
use std::collections::HashSet;

pub fn run() {
    println!("Day 6 Solutions");
    println!("---------------");
    let input = read_input_for_day(6);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Cell = char;
type Row = Vec<Cell>;
type Grid = Vec<Row>;

fn parse_input(input: &String) -> Grid {
    input
        .split('\n')
        .map(|line| line.chars().collect::<Row>())
        .collect::<Grid>()
}

type Coord = (usize, usize);
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Pos {
    coord: Coord,
    dir: char,
}

impl Pos {
    fn new(coord: Coord, dir: char) -> Self {
        Self {
            coord: coord,
            dir: dir,
        }
    }
}

fn find_start_position(grid: &Grid) -> Pos {
    let guard_positions = vec!['v', '<', '^', '>'];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if guard_positions.contains(&grid[i][j]) {
                return Pos::new((i, j), grid[i][j]);
            }
        }
    }

    panic!("start position not found!");
}

fn turn_90(pos: &Pos) -> Pos {
    Pos::new(
        pos.coord,
        match pos.dir {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => panic!("Unexpected direction: {}", pos.dir),
        },
    )
}

fn take_step(pos: &Pos) -> Pos {
    Pos::new(
        match pos.dir {
            '>' => (pos.coord.0, pos.coord.1 + 1),
            'v' => (pos.coord.0 + 1, pos.coord.1),
            '<' => (pos.coord.0, pos.coord.1 - 1),
            '^' => (pos.coord.0 - 1, pos.coord.1),
            _ => panic!("Unexpected direction: {}", pos.dir),
        },
        pos.dir,
    )
}

fn hits_obstacle(pos: &Pos, grid: &Grid) -> bool {
    grid[pos.coord.0][pos.coord.1] == '#'
}

fn take_step_or_turn(pos: &Pos, grid: &Grid) -> Pos {
    let next_pos = take_step(&pos);
    if hits_obstacle(&next_pos, &grid) {
        turn_90(&pos)
    } else {
        next_pos
    }
}

fn exits_grid(pos: &Pos, grid: &Grid) -> bool {
    match pos.dir {
        '>' => pos.coord.1 >= grid[0].len() - 1,
        'v' => pos.coord.0 >= grid.len() - 1,
        '<' => pos.coord.1 <= 0,
        '^' => pos.coord.0 <= 0,
        _ => false,
    }
}

fn solve_part1(input: &String) -> String {
    let grid = parse_input(input);
    let mut pos = find_start_position(&grid);
    let mut unique_coords = HashSet::<Coord>::new();
    unique_coords.insert(pos.coord);
    while !exits_grid(&pos, &grid) {
        pos = take_step_or_turn(&pos, &grid);
        unique_coords.insert(pos.coord);
    }

    unique_coords.len().to_string()
}

fn coords_around(coord: &Coord, grid: &Grid) -> Vec<Coord> {
    let mut coords = Vec::<Coord>::new();

    for di in [-1, 0, 1] {
        let i = coord.0 as isize + di;
        for dj in [-1, 0, 1] {
            let j = coord.1 as isize + dj;
            if i > 0 && i < grid.len() as isize && j > 0 && j < grid[0].len() as isize {
                let i = i as usize;
                let j = j as usize;
                if grid[i][j] == '.' && (i, j) != *coord {
                    coords.push((i, j));
                }
            }
        }
    }

    coords
}

fn solve_part2(input: &String) -> String {
    let grid = parse_input(input);
    let mut pos = find_start_position(&grid);
    let mut unique_coords = HashSet::<Coord>::new();
    unique_coords.insert(pos.coord);
    let mut columns = HashSet::<usize>::new();
    let mut rows = HashSet::<usize>::new();
    while !exits_grid(&pos, &grid) {
        pos = take_step_or_turn(&pos, &grid);
        unique_coords.insert(pos.coord);
        match pos.dir {
            '^' | 'v' => columns.insert(pos.coord.1),
            '<' | '>' => rows.insert(pos.coord.0),
            _ => false,
        };
    }

    let candidates = unique_coords
        .iter()
        .filter(|coord| rows.contains(&coord.0) || columns.contains(&coord.1))
        .map(|coord| coords_around(coord, &grid))
        .flatten()
        .collect::<Vec<Coord>>();

    let mut new_obstacles = HashSet::<Coord>::new();
    for coord in candidates {
        let mut grid = parse_input(input);
        grid[coord.0][coord.1] = '#';
        let mut pos = find_start_position(&grid);
        let mut unique_pos = HashSet::<Pos>::new();
        unique_pos.insert(pos.clone());
        while !exits_grid(&pos, &grid) {
            pos = take_step_or_turn(&pos, &grid);
            if unique_pos.contains(&pos) {
                new_obstacles.insert(coord);
                break;
            }

            unique_pos.insert(pos.clone());
        }
    }

    new_obstacles.len().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let grid = parse_input(&get_input(0));

        assert_eq!(grid[0][0], '.');
        assert_eq!(grid[3][2], '#');
    }

    #[test]
    fn test_find_start_position() {
        let grid = parse_input(&get_input(0));
        let pos = find_start_position(&grid);
        assert_eq!(pos.coord, (6, 4));
    }

    #[test]
    fn test_exits_grid() {
        let grid = parse_input(&get_input(0));
        for pos in vec![
            Pos::new((0, 0), '<'),
            Pos::new((0, 0), '^'),
            Pos::new((5, 9), '>'),
        ] {
            assert!(exits_grid(&pos, &grid));
        }

        for pos in vec![
            Pos::new((0, 0), '>'),
            Pos::new((0, 0), 'v'),
            Pos::new((4, 4), '<'),
            Pos::new((4, 4), '^'),
        ] {
            assert!(!exits_grid(&pos, &grid));
        }
    }

    #[test]
    fn test_take_step_or_turn() {
        let grid = parse_input(&get_input(0));
        // Step
        for (start, end) in vec![
            (Pos::new((0, 0), '>'), Pos::new((0, 1), '>')),
            (Pos::new((0, 0), 'v'), Pos::new((1, 0), 'v')),
            (Pos::new((4, 4), '<'), Pos::new((4, 3), '<')),
            (Pos::new((4, 4), '^'), Pos::new((3, 4), '^')),
        ] {
            let actual_end = take_step_or_turn(&start, &grid);
            assert_eq!(end, actual_end);
        }

        //Turn
        for (start, end) in vec![
            (Pos::new((0, 3), '>'), Pos::new((0, 3), 'v')),
            (Pos::new((0, 9), 'v'), Pos::new((0, 9), '<')),
            (Pos::new((3, 3), '<'), Pos::new((3, 3), '^')),
            (Pos::new((4, 2), '^'), Pos::new((4, 2), '>')),
        ] {
            let actual_end = take_step_or_turn(&start, &grid);
            assert_eq!(end, actual_end);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "41");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "6");
    }
}
