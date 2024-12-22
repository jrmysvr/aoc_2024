use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 15 Solutions");
    println!("---------------");
    let input = read_input_for_day(15);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = isize;
#[derive(Debug, Clone, Eq, PartialEq)]
struct Thing {
    c: char,
    i: Num,
    j: Num,
}

impl Thing {
    fn new(c: char, i: Num, j: Num) -> Self {
        Self { c: c, i: i, j: j }
    }

    fn loc(&self) -> Loc {
        (self.i, self.j)
    }

    fn move_to(&self, loc: Loc) -> Thing {
        Thing::new(self.c, loc.0, loc.1)
    }
}

type Things = Vec<Thing>;

type Row = Vec<char>;
type Grid = Vec<Row>;
type Direction = char;
type Moves = Vec<Direction>;
type Loc = (Num, Num);

fn move_loc(loc: Loc, direction: Direction) -> Loc {
    match direction {
        '>' => (loc.0, loc.1 + 1),
        'v' => (loc.0 + 1, loc.1),
        '<' => (loc.0, loc.1 - 1),
        '^' => (loc.0 - 1, loc.1),
        _ => panic!("Unknown direction: {direction}"),
    }
}
fn find_in(grid: &Grid, elem: char) -> Vec<Loc> {
    let mut output = Vec::<Loc>::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == elem {
                output.push((i as Num, j as Num));
            }
        }
    }

    output
}

fn find_things_in(grid: &Grid, elem: char) -> Things {
    find_in(grid, elem)
        .iter()
        .map(|loc| Thing::new(elem, loc.0 as Num, loc.1 as Num))
        .collect::<Things>()
}

fn parse_input(input: &String) -> (Thing, Grid, Moves) {
    let grid_str = input.split("\n\n").nth(0).unwrap();
    let move_str = input.split("\n\n").nth(1).unwrap();
    let grid = grid_str
        .split('\n')
        .map(|row| row.chars().collect::<Row>())
        .collect::<Grid>();
    let robot = find_things_in(&grid, '@')[0].clone();

    let moves = move_str.chars().filter(|m| *m != '\n').collect::<Moves>();

    (robot, grid, moves)
}

fn in_bounds(loc: Loc, grid: &Grid) -> bool {
    loc.0 >= 0 && loc.0 < grid.len() as Num && loc.1 >= 0 && loc.1 < grid[0].len() as Num
}

fn get_adjacent_to(thing: &Thing, direction: Direction, grid: &Grid) -> Thing {
    let loc = move_loc(thing.loc(), direction);
    if in_bounds(loc, grid) {
        return get_thing_at(loc, grid);
    }

    thing.clone()
}

fn move_thing(thing: &Thing, grid: &Grid, direction: Direction) -> Option<(Thing, Grid)> {
    if in_bounds(move_loc(thing.loc(), direction), grid) {
        let thing_to_push = get_adjacent_to(thing, direction, grid);
        match thing_to_push.c {
            // Move!
            '.' => {
                let moved_thing = thing.move_to(thing_to_push.loc());
                let mut new_grid = grid.clone();
                new_grid[thing.i as usize][thing.j as usize] = '.';
                new_grid[moved_thing.i as usize][moved_thing.j as usize] = moved_thing.c;
                return Some((moved_thing, new_grid));
            }
            // Need to push stuff
            'O' => {
                if let Some((_, grid)) = move_thing(&thing_to_push, grid, direction) {
                    return move_thing(thing, &grid, direction);
                }
            }
            // At a wall, so do nothing
            '#' => {}
            _ => {}
        }
    }

    None
}

fn move_thing_2(thing: &Thing, grid: &Grid, direction: Direction) -> Option<(Thing, Grid)> {
    if in_bounds(move_loc(thing.loc(), direction), grid) {
        let thing_to_push = get_adjacent_to(thing, direction, grid);
        match thing_to_push.c {
            // Move!
            '.' => {
                let moved_thing = thing.move_to(thing_to_push.loc());
                let mut new_grid = grid.clone();
                new_grid[thing.i as usize][thing.j as usize] = '.';
                new_grid[moved_thing.i as usize][moved_thing.j as usize] = moved_thing.c;
                return Some((moved_thing, new_grid));
            }
            // Need to push stuff
            '[' | ']' => match direction {
                '<' | '>' => {
                    if let Some((_, grid)) = move_thing_2(&thing_to_push, grid, direction) {
                        return move_thing_2(thing, &grid, direction);
                    }
                }
                '^' | 'v' => {
                    let lr = if thing_to_push.c == '[' { '>' } else { '<' };
                    let other_side = get_adjacent_to(&thing_to_push, lr, grid);
                    if let Some((_, grid)) = move_thing_2(&other_side, &grid, direction) {
                        if let Some((_, grid)) = move_thing_2(&thing_to_push, &grid, direction) {
                            return move_thing_2(thing, &grid, direction);
                        }
                    }
                }
                _ => {}
            },
            // At a wall, so do nothing
            '#' => {}
            _ => {}
        }
    }

    None
}

fn get_thing_at(loc: Loc, grid: &Grid) -> Thing {
    let loc0 = loc.0 as usize;
    let loc1 = loc.1 as usize;
    Thing::new(grid[loc0][loc1], loc.0, loc.1)
}

fn solve_part1(input: &String) -> String {
    let (mut robot, mut grid, moves) = parse_input(input);

    for direction in moves {
        if let Some(robot_grid) = move_thing(&robot, &grid, direction) {
            (robot, grid) = robot_grid;
        }
    }

    let boxes = find_things_in(&grid, 'O');
    boxes
        .iter()
        .map(|b| 100 * b.i + b.j)
        .fold(0, |acc, gps_coor| acc + gps_coor)
        .to_string()
}

fn scale_grid(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new();
    for row in grid {
        let mut new_row = Row::new();
        for elem in row {
            match elem {
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => panic!("Unexpected element: {elem}"),
            }
        }
        new_grid.push(new_row);
    }

    new_grid
}

fn solve_part2(input: &String) -> String {
    let (_, grid, moves) = parse_input(input);
    let mut grid = scale_grid(&grid);
    let mut robot = find_things_in(&grid, '@')[0].clone();

    for direction in moves {
        if let Some(robot_grid) = move_thing_2(&robot, &grid, direction) {
            (robot, grid) = robot_grid;
        }
    }

    let boxes = find_things_in(&grid, '[');
    boxes
        .iter()
        .map(|b| 100 * b.i + b.j)
        .fold(0, |acc, gps_coor| acc + gps_coor)
        .to_string()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use super::*;

    const INPUT: &[&str] = &[
        "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    fn render_grid(grid: &Grid) {
        println!("");
        println!(
            "{}",
            grid.into_iter()
                .map(|row| row.into_iter().collect::<String>())
                .join("\n")
        );
    }

    #[test]
    fn test_parse_input() {
        let (robot, grid, moves) = parse_input(&get_input(1));
        assert_eq!(robot, Thing::new('@', 2, 2));
        assert_eq!(grid[0][0], '#');
        assert_eq!(moves[0], '<');
    }

    #[test]
    fn test_move_stuff() {
        // Move right
        let loc = move_loc((1, 1), '>');
        assert_eq!(loc, (1, 2), "failed moving right");

        // Move down
        let loc = move_loc((1, 1), 'v');
        assert_eq!(loc, (2, 1), "failed moving down");

        // Move left
        let loc = move_loc((1, 1), '<');
        assert_eq!(loc, (1, 0), "failed moving left");

        // Move up
        let loc = move_loc((1, 1), '^');
        assert_eq!(loc, (0, 1), "failed moving up");

        let (robot, grid, _) = parse_input(&get_input(1));
        let robot_grid = move_thing(&robot, &grid, '>');
        assert!(robot_grid.is_some());
        let Some((robot, grid)) = robot_grid else {
            panic!("Unexpected")
        };
        assert_eq!(robot.i, 2);
        assert_eq!(robot.j, 3);
        assert_eq!(get_thing_at(robot.loc(), &grid), robot);
    }

    #[test]
    fn test_push_stuff() {
        let (robot, grid, _) = parse_input(&get_input(1));
        assert!(move_thing(&robot, &grid, '<').is_none());

        let robot_grid = move_thing(&robot, &grid, '^');
        let Some((robot, grid)) = robot_grid else {
            panic!("Unexpected")
        };

        assert_eq!(robot.i, 1);
        assert_eq!(robot.j, 2);
        let loc = (robot.i - 1, robot.j);
        let w = Thing::new('#', loc.0, loc.1);
        assert_eq!(get_thing_at(loc, &grid), w);

        assert!(move_thing(&robot, &grid, '^').is_none());

        let robot_grid = move_thing(&robot, &grid, '>');
        let Some((robot, grid)) = robot_grid else {
            panic!("Unexpected")
        };
        let robot_grid = move_thing(&robot, &grid, '>');
        let Some((robot, grid)) = robot_grid else {
            panic!("Unexpected")
        };

        assert!(move_thing(&robot, &grid, '>').is_none());
        let robot_grid = move_thing(&robot, &grid, 'v');
        let Some((robot, grid)) = robot_grid else {
            panic!("Unexpected")
        };

        assert!(move_thing(&robot, &grid, 'v').is_none());
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "10092");
        assert_eq!(solve_part1(&get_input(1)), "2028");
    }

    #[test]
    fn test_push_stuff_2() {
        let (_, grid, _) = parse_input(&get_input(2));
        let mut grid = scale_grid(&grid);
        let mut robot = find_things_in(&grid, '@')[0].clone();

        render_grid(&grid);
        let robot_grid = move_thing_2(&robot, &grid, '<');
        let Some((r, g)) = robot_grid else {
            panic!("Unexpected")
        };
        robot = r;
        grid = g;

        assert_eq!(robot.i, 3);
        assert_eq!(robot.j, 9);
        let loc = (robot.i - 1, robot.j);
        let w = Thing::new('.', loc.0, loc.1);
        assert_eq!(get_thing_at(loc, &grid), w);
        render_grid(&grid);

        for direction in vec!['v', 'v', '<', '<', '^'] {
            let Some((r, g)) = move_thing_2(&robot, &grid, direction) else {
                panic!("Unexpected")
            };
            robot = r;
            grid = g;
            render_grid(&grid);
        }
        assert!(move_thing_2(&robot, &grid, '^').is_none());
        render_grid(&grid);
        for direction in vec!['<', '<', '^', '^'] {
            let Some((r, g)) = move_thing_2(&robot, &grid, direction) else {
                panic!("Unexpected")
            };

            robot = r;
            grid = g;
            render_grid(&grid);
        }
        assert!(move_thing_2(&robot, &grid, '^').is_none());
        render_grid(&grid);
    }

    #[test]
    fn test_scale_grid() {
        let (_, grid, _) = parse_input(&get_input(2));
        let grid = scale_grid(&grid);
        assert_eq!(grid.len(), 7);
        assert_eq!(grid[0].len(), 14);
        assert_eq!(grid[3].len(), 14);
        assert_eq!(grid[3][6], '[');
        assert_eq!(grid[3][7], ']');
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "9021");
    }
}
