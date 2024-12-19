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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Thing {
    i: usize,
    j: usize,
}

impl Thing {
    fn new(i: usize, j: usize) -> Self {
        Self { i: i, j: j }
    }

    fn go(&mut self, direction: char) {
        match direction {
            '>' => { self.j += 1; },
            'v' => { self.i += 1; },
            '<' => { self.j -= 1; },
            '^' => { self.i -= 1; },
            _ => panic!("Unsupported direction: {direction}"),
        }
    }
}

type Row = Vec<char>;
type Grid = Vec<Row>;
type Direction = char;
type Moves = Vec<Direction>;
type Loc = (usize, usize);
fn find_in(grid: &Grid, elem: char) -> Vec<Loc> {
    let mut output = Vec::<Loc>::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == elem {
                output.push((i, j));
            }
        }
    }

    output
}

fn parse_input(input: &String) -> (Thing, Grid, Moves) {
    let grid_str = input.split("\n\n").nth(0).unwrap();
    let move_str = input.split("\n\n").nth(1).unwrap();
    let grid = grid_str
        .split('\n')
        .map(|row| row.chars().collect::<Row>())
        .collect::<Grid>();
    let (ri, rj) = find_in(&grid, '@')[0];
    let robot = Thing::new(ri, rj);

    let moves = move_str.chars().collect::<Moves>();

    (robot, grid, moves)
}

fn move_robot_in(grid: &mut Grid, direction: Direction) {

}

fn solve_part1(input: &String) -> String {
    let (_robot, mut grid, moves) = parse_input(input);

    for direction in moves {
        move_robot_in(&mut grid, direction);
    }

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
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let (robot, grid, moves) = parse_input(&get_input(1));
        assert_eq!(robot, Thing::new(2, 2));
        assert_eq!(grid[0][0], '#');
        assert_eq!(moves[0], '<');
    }

    #[test]
    fn test_move_thing() {
        let mut thing = Thing::new(1,1);
        // Move right
        thing.go('>');
        assert_eq!(thing, Thing::new(1,2), "failed moving right");

        let mut thing = Thing::new(1,1);
        // Move down 
        thing.go('v');
        assert_eq!(thing, Thing::new(2,1), "failed moving down");

        let mut thing = Thing::new(1,1);
        // Move left
        thing.go('<');
        assert_eq!(thing, Thing::new(1,0), "failed moving left");

        let mut thing = Thing::new(1,1);
        // Move up 
        thing.go('^');
        assert_eq!(thing, Thing::new(0,1), "failed moving up");
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
