use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut fish = include_str!("./input.txt")
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..80 {
        fish = fish.iter().fold(vec![], |mut acc, age| {
            let age = if *age == 0 {
                acc.push(8);
                6
            } else {
                age - 1
            };
            acc.push(age);

            acc
        })
    }
    println!("Part 1: {}", fish.len());
}

fn part2() {
    let mut fish_buckets = include_str!("./input.txt")
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut acc, age| {
            *acc.entry(age).or_insert(0usize) += 1;
            acc
        });
    let days = 256;
    for _ in 0..days {
        let mut buckets = HashMap::new();
        for (&age, &count) in &fish_buckets {
            if age == 0 {
                buckets.insert(8, count);
                *buckets.entry(6).or_insert(0) += count;
            } else {
                *buckets.entry(age - 1).or_insert(0) += count;
            }
        }
        fish_buckets = buckets;
    }

    let fish_count = fish_buckets
        .into_iter()
        .fold(0, |acc, (_age, count)| acc + count);

    println!("Part 5: {}", fish_count);
}
