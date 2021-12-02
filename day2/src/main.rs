use itertools::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let result = include_str!("./input.txt")
        .lines()
        .fold((0, 0), |acc, line| {
            if let Some((dir, val)) = line.split_ascii_whitespace().collect_tuple() {
                let val: i32 = val.parse().unwrap();
                match dir {
                    "forward" => (acc.0 + val, acc.1),
                    "up" => (acc.0, acc.1 - val),
                    "down" => (acc.0, acc.1 + val),
                    _ => acc,
                }
            } else {
                acc
            }
        });

    println!("Part1: {}", result.0 * result.1);
}


fn part2() {
    let result = include_str!("./input.txt")
        .lines()
        .fold((0, 0, 0), |acc, line| {
            if let Some((dir, val)) = line.split_ascii_whitespace().collect_tuple() {
                let val: i32 = val.parse().unwrap();
                match dir {
                    "forward" => (acc.0 + val, acc.1 + val * acc.2, acc.2),
                    "up" => (acc.0, acc.1, acc.2 - val),
                    "down" => (acc.0, acc.1, acc.2 + val),
                    _ => acc,
                }
            } else {
                acc
            }
        });

    println!("Part2: {}", result.0 * result.1);
}

