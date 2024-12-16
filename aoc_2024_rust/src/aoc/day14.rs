use crate::aoc::input::read_input_for_day;
use fancy_regex::Regex;

pub fn run() {
    println!("Day 14 Solutions");
    println!("---------------");
    let input = read_input_for_day(14);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = isize;

#[derive(Debug, Eq, PartialEq, Clone)]
struct XY {
    x: Num,
    y: Num,
}

impl XY {
    fn new(x: Num, y: Num) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Robot {
    p: XY,
    v: XY,
    map_dim: XY,
}

impl Robot {
    fn from_xys(p: XY, v: XY, map_dim: XY) -> Self {
        Self {
            p: p,
            v: v,
            map_dim: map_dim,
        }
    }

    fn new(px: Num, py: Num, vx: Num, vy: Num, map_dim: XY) -> Self {
        Robot::from_xys(XY::new(px, py), XY::new(vx, vy), map_dim)
    }

    fn tick(&mut self) {
        let mut x = self.p.x + self.v.x;
        let mut y = self.p.y + self.v.y;
        if x < 0 {
            x += self.map_dim.x;
        } else if x >= self.map_dim.x {
            x -= self.map_dim.x;
        }

        if y < 0 {
            y += self.map_dim.y;
        } else if y >= self.map_dim.y {
            y -= self.map_dim.y;
        }
        self.p.x = x;
        self.p.y = y;
    }
}

type Robots = Vec<Robot>;

fn parse_input(input: &String, map_dim: &XY) -> Robots {
    let mut robots = Robots::new();
    let robots_re = Regex::new(r"(?<=[pv]=)(?<x>-?[0-9]+),(?<y>-?[0-9]+)").unwrap();
    for line in input.split('\n') {
        let mut caps = robots_re.captures_iter(line);
        let cap_p = caps.nth(0).unwrap().unwrap();
        let cap_v = caps.nth(0).unwrap().unwrap();
        let px = cap_p["x"].parse::<Num>().unwrap();
        let py = cap_p["y"].parse::<Num>().unwrap();
        let vx = cap_v["x"].parse::<Num>().unwrap();
        let vy = cap_v["y"].parse::<Num>().unwrap();
        robots.push(Robot::new(px, py, vx, vy, (*map_dim).clone()));
    }

    robots
}

fn render_robot_map(robots: &Robots) {
    let mut map: Vec<Vec<String>> = Vec::new();
    let nrows = robots[0].map_dim.y as usize;
    let ncols = robots[0].map_dim.x as usize;
    for i in 0..nrows {
        let mut row = Vec::new();
        for j in 0..ncols {
            let count = count_robots_in(XY::new(j as isize, i as isize), &robots);
            if count > 0 {
                row.push(count.to_string());
            } else {
                row.push(".".to_string());
            }
        }
        map.push(row);
    }

    println!(
        "{}",
        map.into_iter()
            .map(|row| row.into_iter().collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("");
}

fn count_robots_in(loc: XY, robots: &Robots) -> Num {
    robots
        .iter()
        .filter(|robot| loc.x == robot.p.x && loc.y == robot.p.y)
        .count() as Num
}

fn calc_locs_in_quadrant(quadrant_ix: usize, map_dim: &XY) -> Vec<XY> {
    let (xx, yy) = match quadrant_ix {
        0 => ((0, map_dim.x / 2), (0, map_dim.y / 2)),
        1 => ((map_dim.x / 2 + 1, map_dim.x), (0, map_dim.y / 2)),
        2 => ((0, map_dim.x / 2), (map_dim.y / 2 + 1, map_dim.y)),
        3 => (
            (map_dim.x / 2 + 1, map_dim.x),
            (map_dim.y / 2 + 1, map_dim.y),
        ),
        _ => panic!("Unsupported quadrant: {quadrant_ix}"),
    };

    let mut locs = Vec::<XY>::new();
    for x in xx.0..xx.1 {
        for y in yy.0..yy.1 {
            locs.push(XY::new(x, y));
        }
    }

    locs
}

fn count_robots_in_quadrant(quadrant_ix: usize, robots: &Robots) -> Num {
    // Assume all robots have the same `map_dim`
    let map_dim = robots[0].map_dim.clone();
    let locs_in_quadrant = calc_locs_in_quadrant(quadrant_ix, &map_dim);
    locs_in_quadrant
        .into_iter()
        .map(|loc| count_robots_in(loc, robots))
        .fold(0, |acc, count| acc + count) as Num
}

fn solve_part1_with_map_dim(input: &String, map_dim: &XY) -> String {
    let mut robots = parse_input(input, &map_dim);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.tick();
        }
    }
    
    (0..=3)
        .map(|ix| count_robots_in_quadrant(ix, &robots))
        .fold(1, |acc, count| acc * count)
        .to_string()
}

fn solve_part1(input: &String) -> String {
    let map_dim = XY::new(101, 103);
    solve_part1_with_map_dim(input, &map_dim)
}

fn solve_part2(input: &String) -> String {
    /*
    let map_dim = XY::new(101, 103);
    let mut robots = parse_input(input, &map_dim);

    let n_ticks = 200;
    for _ in 0..n_ticks {
        for robot in robots.iter_mut() {
            robot.tick();
        }
        render_robot_map(&robots);
    }
    
    //render_robot_map(&robots);
    */

    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &["
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_input() {
        let map_dim = XY::new(11, 7);
        let robots = parse_input(&get_input(0), &map_dim);
        let p0 = XY::new(0, 4);
        let v0 = XY::new(3, -3);
        let p_end = XY::new(9, 5);
        let v_end = XY::new(-3, -3);
        let robot0 = Robot::from_xys(p0.clone(), v0.clone(), map_dim.clone());
        let robot_end = Robot::from_xys(p_end.clone(), v_end.clone(), map_dim.clone());
        assert_eq!(robots[0], robot0);
        assert_eq!(robots[robots.len() - 1], robot_end);
    }

    #[test]
    fn test_robot_tick() {
        let map_dim = XY::new(11, 7);
        let p0 = XY::new(0, 4);
        let v0 = XY::new(3, -3);
        let mut robot0 = Robot::from_xys(p0.clone(), v0.clone(), map_dim.clone());
        robot0.tick();
        assert_eq!(robot0.p.x, 3);
        assert_eq!(robot0.p.y, 1);
    }

    #[test]
    fn test_robot_tick_wraps() {
        let map_dim = XY::new(11, 7);
        let p0 = XY::new(0, 0);
        let v0 = XY::new(-3, -3);
        let mut robot0 = Robot::from_xys(p0.clone(), v0.clone(), map_dim.clone());
        robot0.tick();
        assert_eq!(robot0.p.x, 8);
        assert_eq!(robot0.p.y, 4);

        let p0 = XY::new(2, 4);
        let v0 = XY::new(2, -3);
        let mut robot0 = Robot::from_xys(p0.clone(), v0.clone(), map_dim.clone());

        robot0.tick();
        assert_eq!(robot0.p.x, 4);
        assert_eq!(robot0.p.y, 1);

        robot0.tick();
        assert_eq!(robot0.p.x, 6);
        assert_eq!(robot0.p.y, 5);

        robot0.tick();
        assert_eq!(robot0.p.x, 8);
        assert_eq!(robot0.p.y, 2);
    }

    #[test]
    fn test_robot_counts() {
        let map_dim = XY::new(11, 7);
        let mut robots = vec![Robot::new(2, 4, 2, -3, map_dim.clone())];
        assert_eq!(count_robots_in(XY::new(2, 4), &robots), 1);
        for _ in 0..1 {
            for robot in robots.iter_mut() {
                robot.tick();
            }
        }

        assert_eq!(count_robots_in(XY::new(4, 1), &robots), 1);

        let mut robots = parse_input(&get_input(0), &map_dim);
        render_robot_map(&robots);

        for _ in 0..100 {
            for robot in robots.iter_mut() {
                robot.tick();
            }
        }
        render_robot_map(&robots);
        assert_eq!(count_robots_in(XY::new(0, 2), &robots), 1);
        assert_eq!(count_robots_in(XY::new(6, 0), &robots), 2);
    }

    #[test]
    fn test_robot_quadrant_count() {
        let map_dim = XY::new(11, 7);
        let mut robots = parse_input(&get_input(0), &map_dim);
        for _ in 0..100 {
            for robot in robots.iter_mut() {
                robot.tick();
            }
        }
        render_robot_map(&robots);
        assert_eq!(count_robots_in_quadrant(0, &robots), 1, "quadrant 0 failed");
        assert_eq!(count_robots_in_quadrant(1, &robots), 3, "quadrant 1 failed");
        assert_eq!(count_robots_in_quadrant(2, &robots), 4, "quadrant 2 failed");
        assert_eq!(count_robots_in_quadrant(3, &robots), 1, "quadrant 3 failed");
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(
            solve_part1_with_map_dim(&get_input(0), &XY::new(11, 7)),
            "12"
        );
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
