use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let positions = include_str!("./input.txt")
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut position_counts = positions
        .iter()
        .fold(HashMap::new(), |mut acc, pos| {
            *acc.entry(pos).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .collect::<Vec<_>>();
    position_counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let fuels = position_counts
        .iter()
        .map(|(&pos, _)| positions.iter().fold(0, |acc, p| acc + i32::abs(p - pos)))
        .min();
    println!("Part 1: {:?}", fuels);
}

fn part2() {
    let positions = include_str!("./input.txt")
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let max_pos = positions.iter().max().unwrap().to_owned();
    let min_pos = positions.iter().min().unwrap().to_owned();
    let fuels = (min_pos..max_pos)
        .map(|pos| {
            positions.iter().fold(0, |acc, p| {
                let diff = i32::abs(p - pos);
                let fuel = (diff.pow(2) + diff) / 2;
                acc + fuel
            })
        })
        .min();
    println!("Part 2: {:?}", fuels);
}
