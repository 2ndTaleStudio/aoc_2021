use itertools::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let result = include_str!("./input.txt")
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .enumerate()
        .fold((0, 0), |mut acc, entry| {
            let current: i32 = entry.1;
            if entry.0 > 0 && current > acc.1 {
                acc.0 += 1;
            }
            (acc.0, current)
        });

    println!("Part1: {}", result.0);
}

fn part2() {
    let result = include_str!("./input.txt")
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|wnd| wnd.0 + wnd.1 + wnd.2)
        .enumerate()
        .fold((0, 0), |mut acc, entry| {
            let current: i32 = entry.1;
            if entry.0 > 0 && current > acc.1 {
                acc.0 += 1;
            }
            (acc.0, current)
        });

    println!("Part2: {}", result.0);
}
