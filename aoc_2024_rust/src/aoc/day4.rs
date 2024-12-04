use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 4 Solutions");
    println!("---------------");
    let input = read_input_for_day(4);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Row = Vec<char>;
type Grid = Vec<Row>;

fn parse_input(input: &String) -> Grid {
    input
        .trim()
        .split('\n')
        .map(|line| line.chars().collect::<Row>())
        .collect::<Grid>()
}

type Indexes = Vec<(usize, usize)>;

fn indexes_right((i, j): (usize, usize), length: usize, grid: &Grid) -> Indexes {
    if j + length <= grid[0].len() {
        (0..length).map(|d| (i, j + d)).collect()
    } else {
        vec![]
    }
}

fn indexes_left((i, j): (usize, usize), length: usize, _grid: &Grid) -> Indexes {
    if j >= length - 1 {
        (0..length).map(|d| (i, j - d)).collect()
    } else {
        vec![]
    }
}

fn indexes_up((i, j): (usize, usize), length: usize, _grid: &Grid) -> Indexes {
    if i >= length - 1 {
        (0..length).map(|d| (i - d, j)).collect()
    } else {
        vec![]
    }
}

fn indexes_down((i, j): (usize, usize), length: usize, grid: &Grid) -> Indexes {
    if i + length <= grid.len() {
        (0..length).map(|d| (i + d, j)).collect()
    } else {
        vec![]
    }
}

fn indexes_diag_up_right((i, j): (usize, usize), length: usize, grid: &Grid) -> Indexes {
    if i >= length - 1 && j + length <= grid[0].len() {
        (0..length).map(|d| (i - d, j + d)).collect()
    } else {
        vec![]
    }
}

fn indexes_diag_up_left((i, j): (usize, usize), length: usize, _grid: &Grid) -> Indexes {
    if i >= length - 1 && j >= length - 1 {
        (0..length).map(|d| (i - d, j - d)).collect()
    } else {
        vec![]
    }
}

fn indexes_diag_down_right((i, j): (usize, usize), length: usize, grid: &Grid) -> Indexes {
    if i + length <= grid.len() && j + length <= grid[0].len() {
        (0..length).map(|d| (i + d, j + d)).collect()
    } else {
        vec![]
    }
}

fn indexes_diag_down_left((i, j): (usize, usize), length: usize, grid: &Grid) -> Indexes {
    if i + length <= grid.len() && j >= length - 1 {
        (0..length).map(|d| (i + d, j - d)).collect()
    } else {
        vec![]
    }
}

fn count_n_xmas_at(ij: (usize, usize), grid: &Grid) -> u32 {
    const XMAS: &str = "XMAS";

    vec![
        indexes_left,
        indexes_right,
        indexes_up,
        indexes_down,
        indexes_diag_up_left,
        indexes_diag_up_right,
        indexes_diag_down_left,
        indexes_diag_down_right,
    ]
    .into_iter()
    .map(|indexes| {
        indexes(ij, XMAS.len(), grid)
            .into_iter()
            .map(|(i, j)| grid[i][j])
            .collect::<String>()
    })
    .filter(|word| word.trim() == XMAS)
    .count() as u32
}

fn count_n_x_mas_at((i, j): (usize, usize), grid: &Grid) -> u32 {
    let x_mas = vec!["MSAMS", "SSAMM", "SMASM", "MMASS"];
    if i > 0 && j > 0 && i < grid.len() - 1 && j < grid[0].len() - 1 {
        let kernel = vec![
            (i - 1, j - 1),
            (i - 1, j + 1),
            (i, j),
            (i + 1, j - 1),
            (i + 1, j + 1),
        ];
        let word = kernel
            .into_iter()
            .map(|(i, j)| grid[i][j])
            .collect::<String>();

        return if x_mas.contains(&word.as_str()) { 1 } else { 0 };
    }

    0
}

fn solve_part1(input: &String) -> String {
    let grid = parse_input(input);
    /*
    let mut n_xmas = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            n_xmas += count_n_xmas_at((i, j), &grid);
        }
    }
    n_xmas.to_string()
    */
    (0..grid.len())
        .map(|i| {
            (0..grid[0].len())
                .map(|j| (i, j))
                .fold(0, |acc, (i, j)| acc + count_n_xmas_at((i, j), &grid))
        })
        .fold(0, |acc, count| acc + count)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    let grid = parse_input(input);
    (1..grid.len())
        .map(|i| {
            (1..grid[0].len())
                .map(|j| (i, j))
                .fold(0, |acc, (i, j)| acc + count_n_x_mas_at((i, j), &grid))
        })
        .fold(0, |acc, count| acc + count)
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 5] = [
        "
..X...
.SAMX.
.A..A.
XMAS.S
.X....
",
        "
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
",
        "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
        // extra tests, part 1
        "
S..S..S
.A.A.A.
..MMM..
SAMXMAS
..MMM..
.A.A.A.
S..S..S
",
        "
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let grid = parse_input(&get_input(0));
        assert_eq!(grid[0][0], '.');
    }

    #[test]
    fn test_count_n_xmas_at() {
        let grid = parse_input(&get_input(1));
        assert_eq!(count_n_xmas_at((9, 9), &grid), 2);
    }

    #[test]
    fn test_indexes() {
        let grid = parse_input(&get_input(0));
        assert_eq!(
            indexes_right((0, 0), 4, &grid),
            vec![(0, 0), (0, 1), (0, 2), (0, 3)]
        );
        assert_eq!(
            indexes_down((0, 0), 4, &grid),
            vec![(0, 0), (1, 0), (2, 0), (3, 0)]
        );
        assert_eq!(
            indexes_down((0, 0), 4, &grid),
            vec![(0, 0), (1, 0), (2, 0), (3, 0)]
        );
        assert_eq!(
            indexes_diag_up_left((4, 4), 4, &grid),
            vec![(4, 4), (3, 3), (2, 2), (1, 1)]
        );
        assert_eq!(
            indexes_diag_up_right((3, 1), 4, &grid),
            vec![(3, 1), (2, 2), (1, 3), (0, 4)]
        );
        assert_eq!(
            indexes_diag_down_right((0, 2), 4, &grid),
            vec![(0, 2), (1, 3), (2, 4), (3, 5)]
        );
        assert_eq!(
            indexes_diag_down_left((1, 3), 4, &grid),
            vec![(1, 3), (2, 2), (3, 1), (4, 0)]
        );
        // Expect Empty
        assert_eq!(indexes_right((4, 4), 4, &grid), vec![]);
        assert_eq!(indexes_right((3, 3), 4, &grid), vec![]);
        assert_eq!(indexes_diag_up_left((2, 2), 4, &grid), vec![]);
        assert_eq!(indexes_diag_up_right((2, 2), 4, &grid), vec![]);
        assert_eq!(indexes_diag_down_right((3, 3), 4, &grid), vec![]);
        assert_eq!(indexes_diag_down_left((2, 2), 4, &grid), vec![]);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "4");
        assert_eq!(solve_part1(&get_input(1)), "18");
        assert_eq!(solve_part1(&get_input(2)), "18");
        assert_eq!(solve_part1(&get_input(3)), "8");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(4)), "9");
    }
}
