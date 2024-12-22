use crate::aoc::input::read_input_for_day;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

pub fn run() {
    println!("Day 16 Solutions");
    println!("---------------");
    let input = read_input_for_day(16);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Row = Vec<char>;
type Maze = Vec<Row>;
fn parse_input(input: &String) -> Maze {
    input
        .split('\n')
        .map(|line| line.chars().collect::<Row>())
        .collect::<Maze>()
}

type Loc = (usize, usize);
type UniqueLocs = HashSet<Loc>;
type Dir = char;
type Neighbor = (Dir, Loc);
type Neighbors = Vec<Neighbor>;

fn cmp_direction(a: Dir, b: Dir) -> Ordering {
    match a {
        '>' => match b {
            'v' | '^' => Ordering::Equal,
            '<' => Ordering::Less,
            _ => Ordering::Greater,
        },
        'v' => match b {
            '<' | '>' => Ordering::Equal,
            '^' => Ordering::Less,
            _ => Ordering::Greater,
        },
        '<' => match b {
            'v' | '^' => Ordering::Equal,
            '>' => Ordering::Less,
            _ => Ordering::Greater,
        },
        '^' => match b {
            '<' | '>' => Ordering::Equal,
            'v' => Ordering::Less,
            _ => Ordering::Greater,
        },
        _ => panic!("Unexpected direction: {a}"),
    }
}

fn neighbors_of(n: Neighbor, maze: &Maze, ignore: UniqueLocs) -> Neighbors {
    let mut neighbors = Neighbors::new();
    // Up
    if (n.1).0 > 0 {
        neighbors.push(('^', ((n.1).0 - 1, (n.1).1)));
    }

    // Right
    if (n.1).1 < maze[0].len() - 1 {
        neighbors.push(('>', ((n.1).0, (n.1).1 + 1)));
    }

    // Down
    if (n.1).0 < maze.len() - 1 {
        neighbors.push(('v', ((n.1).0 + 1, (n.1).1)));
    }

    // Left
    if (n.1).1 > 0 {
        neighbors.push(('<', ((n.1).0, (n.1).1 - 1)));
    }

    let mut neighbors = neighbors
        .into_iter()
        .filter(|neighbor| maze[(neighbor.1).0][(neighbor.1).1] != '#')
        .filter(|neighbor| !ignore.contains(&neighbor.1))
        .collect::<Neighbors>();

    neighbors.sort_by(|a, b| cmp_direction(a.0, b.0));
    neighbors
}

fn find_start_in(maze: &Maze) -> Neighbor {
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                return ('>', (i, j));
            }
        }
    }
    panic!("Didn't find the start position!");
}

fn render_maze(maze: &Maze) {
    for _ in maze.clone().iter() {
        print!("\x1b[A");
    }
    print!("\x1b[A");
    print!("\x1b[A");
    println!("");
    println!(
        "{}",
        maze.into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .join("\n")
    );
    println!("");
}

fn render_path_in(maze: &Maze, path: &Path) {
    let mut maze = maze.clone();
    for n in path {
        let loc = n.1;
        let elem = maze[loc.0][loc.1];
        maze[loc.0][loc.1] = match elem {
            'S' | 'E' => elem,
            _ => n.0,
        };
    }
    render_maze(&maze);
}

type Path = Vec<Neighbor>;
type Paths = Vec<Path>;
fn find_paths_from(
    curr: Neighbor,
    maze: &Maze,
    ignore: UniqueLocs,
    ticks: u32,
) -> Vec<(Num, Path)> {
    // Just in case...
    if ticks > (maze.len() * maze[0].len()) as u32 {
        return vec![(1, vec![curr])];
    }
    let mut all_paths = Vec::<(Num, Path)>::new();
    let mut ignore = ignore.clone();
    let mut min_score = Num::MAX;
    for neighbor in neighbors_of(curr, &maze, ignore.clone()) {
        if maze[(neighbor.1).0][(neighbor.1).1] == 'E' {
            let score = 2 + if curr.0 != neighbor.0 { 1000 } else { 0 };
            let path = vec![curr, neighbor];
            all_paths.push((score, path));
            return all_paths;
        }
        ignore.insert(neighbor.1);

        for (score, mut path) in find_paths_from(neighbor, maze, ignore.clone(), ticks + 1) {
            let end_loc = path[path.len() - 1].1;
            if maze[end_loc.0][end_loc.1] == 'E' && min_score >= score {
                path.insert(0, curr);
                let score = score + if curr.0 != neighbor.0 { 1000 } else { 0 };
                min_score = Num::min(score, min_score);
                all_paths.push((score, path));
            }
        }
    }

    all_paths
}

fn find_paths_through(maze: &Maze) -> Vec<(Num, Path)> {
    let start = find_start_in(maze);
    let mut all_paths = Vec::<(Num, Path)>::new();
    let mut ignore = UniqueLocs::new();
    ignore.insert(start.1);
    let mut min_score = Num::MAX;
    for neighbor in neighbors_of(start, &maze, ignore.clone()) {
        ignore.insert(neighbor.1);
        for (score, mut path) in find_paths_from(neighbor, &maze, ignore.clone(), 0) {
            path.insert(0, start);
            let score = score + if start.0 != neighbor.0 { 1000 } else { 0 };
            min_score = Num::min(score, min_score);
            all_paths.push((score, path));
        }
    }

    all_paths
}

type Num = u32;
fn score_path(path: &Path) -> Num {
    let mut curr = &path[0];
    let mut score = 0;
    for each in path.iter().skip(1) {
        score += 1;
        if each.0 != curr.0 {
            score += 1000;
        }
        curr = each;
    }

    score
}
fn solve_part1(input: &String) -> String {
    let maze = parse_input(input);
    let paths = find_paths_through(&maze);
    paths
        .iter()
        .map(|(_, path)| score_path(path))
        .min()
        .unwrap().to_string()
    /*
    paths
        .iter()
        .map(|(score, _)| score)
        .min()
        .unwrap()
        .to_string()
    */
}

fn solve_part2(_input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &[
        "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let maze = parse_input(&get_input(0));
        assert_eq!(maze[1][13], 'E');
        assert_eq!(maze[13][1], 'S');
    }

    #[test]
    fn test_neighbors_of() {
        let maze = parse_input(&get_input(0));
        let loc = (7, 3);
        let ignore = UniqueLocs::new();
        let neighbors = neighbors_of(('>', loc), &maze, ignore.clone());
        assert!(neighbors.contains(&('^', (6, 3))));
        assert!(neighbors.contains(&('>', (7, 4))));
        assert!(neighbors.contains(&('v', (8, 3))));
        assert!(neighbors.contains(&('<', (7, 2))));

        let loc = (13, 1);
        let neighbors = neighbors_of(('>', loc), &maze, ignore.clone());
        assert!(neighbors.contains(&('^', (12, 1))));
        assert!(neighbors.contains(&('>', (13, 2))));
        assert!(!neighbors.contains(&('v', (14, 1))));
        assert!(!neighbors.contains(&('<', (13, 0))));
    }

    #[test]
    fn test_find_start_in() {
        let maze = parse_input(&get_input(0));
        assert_eq!(find_start_in(&maze), ('>', (13, 1)));
    }

    #[test]
    fn test_find_paths_through() {
        let maze = parse_input(&get_input(0));
        let paths = find_paths_through(&maze);

        render_path_in(&maze, &paths[0].1);
        render_path_in(&maze, &paths[1].1);
        //assert!(false);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "7036");
        assert_eq!(solve_part1(&get_input(1)), "11048");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
