use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 11 Solutions");
    println!("---------------");
    let input = read_input_for_day(11);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Stone = u64;
type Stones = Vec<Stone>;

fn parse_input(input: &String) -> Stones {
    input
        .split(' ')
        .map(|s| s.parse::<Stone>().unwrap())
        .collect::<Stones>()
}

fn stone_from(stones: &Stones) -> Stone {
    let mut pow = 0;
    let mut new_stone = 0;
    let stone_10: Stone = 10;
    for stone in stones.iter().rev() {
        new_stone += stone * stone_10.pow(pow);
        pow += 1;
    }

    new_stone
}

type SplitStone = (Stone, Option<Stone>);
fn split_stone(stone: &Stone) -> SplitStone {
    if *stone == 0 {
        return (0, None);
    }

    let mut new_stones = Stones::new();
    let mut temp_stone = *stone;
    while temp_stone >= 1 {
        let new_stone = temp_stone % 10;
        new_stones.push(new_stone);
        temp_stone /= 10;
    }
    let new_stones = new_stones.into_iter().rev().collect::<Stones>();
    if new_stones.len() % 2 == 0 {
        let mid = new_stones.len() / 2;
        let end = new_stones.len();
        (
            stone_from(&new_stones[0..mid].to_vec()),
            Some(stone_from(&new_stones[mid..end].to_vec())),
        )
    } else {
        (*stone, None)
    }
}

fn convert_stone(stone: &Stone) -> Stone {
    if *stone == 0 {
        1
    } else {
        *stone * 2024
    }
}

fn solve_part1(input: &String) -> String {
    let mut stones = parse_input(input);

    let n_blinks = 25;
    for _ in 0..n_blinks {
        let mut new_stones = Stones::new();
        for stone in stones.iter() {
            let (left_stone, right) = split_stone(&stone);
            if let Some(right_stone) = right {
                new_stones.push(left_stone);
                new_stones.push(right_stone);
            } else {
                let left_stone = convert_stone(&left_stone);
                new_stones.push(left_stone);
            }
        }

        stones = new_stones.clone();
    }

    stones.len().to_string()
}

fn solve_part2(_input: &String) -> String {
    /*
    let mut stones = parse_input(input);

    let mut map = std::collections::HashMap::<Stone, Stones>::new();
    let mut stone_count = stones.len();
    let n_blinks = 75;

    for stone in stones.iter() {
        map.insert(*stone, Stones::new());
    }

    for _ in 0..n_blinks {
        let mapped_stones = map.keys().clone();
        for stone in mapped_stones {
            let (left_stone, right) = split_stone(&stone);
            if let Some(right_stone) = right {
                map.entry(*stone).and_modify(|s| s.push(left_stone));
                map.entry(*stone).and_modify(|s| s.push(right_stone));
            } else {
                let left_stone = convert_stone(&left_stone);
                map.entry(*stone).and_modify(|s| s.push(left_stone));
            }
        }
    }

    stone_count.to_string()
    */
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &["125 17"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let stones = parse_input(&get_input(0));
        assert_eq!(stones, vec![125, 17]);
    }

    #[test]
    fn test_split_stones() {
        for (inpt, expected) in vec![
            (0, (0, None)),
            (10, (1, Some(0))),
            (99, (9, Some(9))),
            (1000, (10, Some(0))),
            (1234, (12, Some(34))),
            (123, (123, None)),
        ] {
            assert_eq!(split_stone(&inpt), expected);
        }
    }

    #[test]
    fn test_stone_from() {
        for (inpt, expected) in vec![
            (vec![0], 0),
            (vec![1, 0], 10),
            (vec![4, 2], 42),
            (vec![1, 2, 3], 123),
        ] {
            assert_eq!(stone_from(&inpt), expected);
        }
    }

    #[test]
    fn test_convert_stone() {}

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "55312");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "0");
    }
}
