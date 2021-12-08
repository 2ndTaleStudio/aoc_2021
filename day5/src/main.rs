use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn part1() {
    let overlapping = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ");
            let mut start = points.next().unwrap().split(",");
            let mut end = points.next().unwrap().split(",");
            Line {
                start: (start.next().unwrap().parse().unwrap(), start.next().unwrap().parse().unwrap()),
                end: (end.next().unwrap().parse().unwrap(), end.next().unwrap().parse().unwrap()),
            }
        })
        .filter(|line| line.start.0 == line.end.0 || line.start.1 == line.end.1)
        .fold(HashMap::new(), |mut acc, line| {
            if line.start.0 == line.end.0 {
                let s = i32::min(line.start.1, line.end.1);
                let e = i32::max(line.start.1, line.end.1);
                for y in s..=e {
                    *acc.entry((line.start.0, y)).or_insert(0) += 1;
                }
            } else {
                let s = i32::min(line.start.0, line.end.0);
                let e = i32::max(line.start.0, line.end.0);
                for x in s..=e {
                    *acc.entry((x, line.start.1)).or_insert(0) += 1;
                }
            }

            acc
        })
        //.drain_filter(|_k, v| *v < 2)
        .iter()
        .filter(|(k, v)| **v > 1)
        .count();
    println!("Part 1: {:?}", overlapping);
}

fn part2() {
    let overlapping = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ");
            let mut start = points.next().unwrap().split(",");
            let mut end = points.next().unwrap().split(",");
            Line {
                start: (start.next().unwrap().parse().unwrap(), start.next().unwrap().parse().unwrap()),
                end: (end.next().unwrap().parse().unwrap(), end.next().unwrap().parse().unwrap()),
            }
        })
        .fold(HashMap::new(), |mut acc, line| {
            let dx = line.end.0 - line.start.0;
            let dy = line.end.1 - line.start.1;
            let steps = i32::max(i32::abs(dx), i32::abs(dy));
            let x_inc = if dx == 0 { 0 } else if dx > 0 { 1 } else { -1 };
            let y_inc = if dy == 0 { 0 } else if dy > 0 { 1 } else { -1 };
            
            for step in 0..=steps {
                *acc.entry((line.start.0 + x_inc * step, line.start.1 + y_inc * step)).or_insert(0) += 1;
            }
            acc
        })
        //.drain_filter(|_k, v| *v < 2)
        .iter()
        .filter(|(k, v)| **v > 1)
        .count();
    println!("Part 2: {:?}", overlapping);
}
