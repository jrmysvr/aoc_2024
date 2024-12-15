use crate::aoc::input::read_input_for_day;
use fancy_regex::Regex;
use nalgebra::{Matrix2, Vector2};

pub fn run() {
    println!("Day 13 Solutions");
    println!("---------------");
    let input = read_input_for_day(13);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = i64;

#[derive(Debug, Eq, PartialEq, Clone)]
struct XY {
    x: Num,
    y: Num,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Game {
    a: XY,
    b: XY,
    prize: XY,
}

impl XY {
    fn new(x: Num, y: Num) -> Self {
        Self { x: x, y: y }
    }
}

impl Game {
    fn new(a: XY, b: XY, prize: XY) -> Self {
        Self {
            a: a,
            b: b,
            prize: prize,
        }
    }
}

fn parse_xy(xy_str: &str, re: &Regex) -> XY {
    let cap = re.captures(xy_str).unwrap().unwrap();
    let x_cap = &cap[1];
    let y_cap = &cap[3];
    XY::new(x_cap.parse::<Num>().unwrap(), y_cap.parse::<Num>().unwrap())
}

type Games = Vec<Game>;

fn parse_input(input: &String) -> Games {
    let button_re = Regex::new(r"(?<=X\+)([0-9]+)(, Y\+)([0-9]+)").unwrap();
    let prize_re = Regex::new(r"(?<=X\=)([0-9]+)(, Y\=)([0-9]+)").unwrap();
    let mut games = Games::new();
    for game_str in input.split("\n\n") {
        let mut game_iter = game_str.split('\n');
        let a = parse_xy(game_iter.nth(0).unwrap(), &button_re);
        let b = parse_xy(game_iter.nth(0).unwrap(), &button_re);
        let prize = parse_xy(game_iter.nth(0).unwrap(), &prize_re);
        games.push(Game::new(a, b, prize));
    }

    games
}

fn calc_cost_of(game: &Game) -> Num {
    let max_presses = 100;
    let mut costs = Vec::<Num>::new();
    for a in 0..max_presses {
        for b in 0..max_presses {
            let x = game.a.x * a + game.b.x * b;
            let y = game.a.y * a + game.b.y * b;
            if x == game.prize.x && y == game.prize.y {
                let cost = 3 * a + b;
                costs.push(cost);
            }
        }
    }

    if let Some(min_cost) = costs.iter().min() {
        *min_cost
    } else {
        0
    }
}

fn solve_part1(input: &String) -> String {
    let games = parse_input(input);
    games
        .iter()
        .map(calc_cost_of)
        .fold(0, |acc, cost| acc + cost)
        .to_string()
}

fn calc_cost_of_mat(game: &Game, d: Num) -> Num {
    /*
    let a = Matrix2::new(game.a.x, game.a.y, game.b.x, game.b.y);
    let b = Vector2::new(d + game.prize.x, d + game.prize.y);
    let decomp = a.lu();
    let x = decomp.solve(&b).expect("Linear resolution failed");

    println!("{x:?}");
    */

    0
}

fn solve_part2(input: &String) -> String {
    let games = parse_input(input);
    let d = 10_000_000_000_000 as Num;
    games
        .iter()
        .map(|game| calc_cost_of_mat(game, d))
        .fold(0, |acc, cost| acc + cost)
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let games = parse_input(&get_input(0));
        let button_a = XY::new(94, 34);
        let button_b = XY::new(22, 67);
        let prize0 = XY::new(8400, 5400);
        let game0 = Game::new(button_a, button_b, prize0);
        assert_eq!(games[0], game0);
    }

    #[test]
    fn test_calc_cost_of() {
        let games = parse_input(&get_input(0));
        assert_eq!(calc_cost_of(&games[0]), 280);
        assert_eq!(calc_cost_of(&games[1]), 0);
        assert_eq!(calc_cost_of(&games[3]), 0);
        assert_eq!(calc_cost_of(&games[2]), 200);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "480");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
