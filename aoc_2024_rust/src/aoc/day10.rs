use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 10 Solutions");
    println!("---------------");
    let input = read_input_for_day(10);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = i32;
type Row = Vec<Num>;
type Topo = Vec<Row>;
fn parse_input(input: &String) -> Topo {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as Num)
                .collect::<Row>()
        })
        .collect::<Topo>()
}

type Pos = (usize, usize);
type Positions = Vec<Pos>;

fn find_positions_with_value(value: Num, topo: &Topo) -> Positions {
    (0..topo.len())
        .map(|i| (0..topo[0].len()).map(|j| (i, j)).collect::<Positions>())
        .flatten()
        .filter(|(i, j)| topo[*i][*j] == value)
        .collect::<Positions>()
}

fn find_trailheads_in(topo: &Topo) -> Positions {
    find_positions_with_value(0, topo)
}

fn neighbors_of(pos: &Pos, topo: &Topo, ignore: &Positions) -> Positions {
    let mut neighbors = Positions::new();
    // Up
    if let Some(pos0) = pos.0.checked_sub(1) {
        neighbors.push((pos0, pos.1));
    }
    // Right
    if pos.1 + 1 < topo[0].len() {
        neighbors.push((pos.0, pos.1 + 1));
    }
    // Down
    if pos.0 + 1 < topo.len() {
        neighbors.push((pos.0 + 1, pos.1));
    }
    // Left
    if let Some(pos1) = pos.1.checked_sub(1) {
        neighbors.push((pos.0, pos1));
    }

    neighbors
        .into_iter()
        .filter(|n| !ignore.contains(&n))
        .collect::<Positions>()
}

fn score(pos: &Pos, topo: &Topo) -> Num {
    let mut ignore = Positions::new();
    let mut all_neighbors = vec![(pos.clone(), neighbors_of(pos, topo, &ignore))];
    ignore.push(pos.clone());
    let mut tops = std::collections::HashSet::<Pos>::new();
    let mut length = all_neighbors.len();
    while length > 0 {
        let (pos, neighbors) = all_neighbors.remove(0);
        let curr = topo[pos.0][pos.1];
        for npos in neighbors {
            let neighbor = topo[npos.0][npos.1];
            if neighbor > curr && (neighbor - curr) == 1 {
                if neighbor == 9 {
                    tops.insert(npos);
                } else {
                    all_neighbors.push((npos.clone(), neighbors_of(&npos, topo, &ignore)));
                }
                ignore.push(npos);
            }
        }
        length = all_neighbors.len();
    }

    tops.len() as Num
}

fn rating(pos: &Pos, topo: &Topo) -> Num {
    let mut score = 0;
    let curr = topo[pos.0][pos.1];
    for npos in neighbors_of(pos, topo, &vec![]) {
        let neighbor = topo[npos.0][npos.1];
        if neighbor > curr && (neighbor - curr) == 1 {
            if neighbor == 9 {
                score += 1;
            } else {
                score += rating(&npos, topo);
            }
        }
    }

    score
}

fn solve_part1(input: &String) -> String {
    let topo = parse_input(input);
    find_trailheads_in(&topo)
        .iter()
        .map(|pos| score(pos, &topo))
        .fold(0, |acc, score| acc + score)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    let topo = parse_input(input);
    find_trailheads_in(&topo)
        .iter()
        .map(|pos| rating(pos, &topo))
        .fold(0, |acc, score| acc + score)
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &[
        "
0123
1234
8765
9876",
        "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        "
012345
123456
234567
345678
406789
567890",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let topo = parse_input(&get_input(0));
        assert_eq!(topo[0], vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_find_trailheads() {
        let topo = parse_input(&get_input(0));
        let trailheads = find_trailheads_in(&topo);
        assert_eq!(trailheads.len(), 1);
        assert_eq!(trailheads[0], (0, 0));
    }

    #[test]
    fn test_score() {
        let topo = parse_input(&get_input(0));
        let trailheads = find_trailheads_in(&topo);
        assert_eq!(score(&trailheads[0], &topo), 1);

        let topo = parse_input(&get_input(1));
        let trailheads = find_trailheads_in(&topo);
        assert_eq!(score(&trailheads[0], &topo), 5);
    }

    #[test]
    fn test_neighbors_of() {
        let topo = parse_input(&get_input(0));
        let empty = vec![];
        for (pos, neighbors) in vec![
            ((0, 0), vec![(0, 1), (1, 0)]),
            ((1, 1), vec![(0, 1), (1, 2), (2, 1), (1, 0)]),
            ((3, 3), vec![(2, 3), (3, 2)]),
            ((2, 2), vec![(1, 2), (2, 3), (3, 2), (2, 1)]),
        ] {
            assert_eq!(neighbors_of(&pos, &topo, &empty), neighbors);
        }
    }

    #[test]
    fn test_rating() {
        let topo = parse_input(&get_input(2));
        let trailheads = find_trailheads_in(&topo);
        assert_eq!(rating(&trailheads[0], &topo), 227);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "1");
        assert_eq!(solve_part1(&get_input(1)), "36");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(2)), "227");
        assert_eq!(solve_part2(&get_input(1)), "81");
    }
}
