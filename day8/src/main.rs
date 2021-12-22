use std::{collections::HashMap, ops::Index, slice::SliceIndex};

fn main() {
    part1();
    part2();
}

fn part1() {
    let unique_outputs = include_str!("./input.txt")
        .lines()
        .map(|line| line.split("| ").nth(1).unwrap().split_ascii_whitespace().collect::<Vec<_>>())
        .fold(0, |acc, output| {
            acc + output.iter().filter(|digit| {
                let segments = digit.len();
                segments == 2 || segments == 4 || segments == 3 || segments == 7
            }).count()
        });
    println!("Part 1: {:?}", unique_outputs);
}

fn part2() {
    let result = include_str!("./input.txt")
        .lines()
        .map(|line| line
                .split(" | ")
                .nth(0)
                .unwrap()
                .split_ascii_whitespace()
                .map(|segments| segments_to_bits(segments))
                .collect::<Vec<_>>())
        .map(|sample| {
            let one = *sample.iter().find(|segments| segments.count_ones() == 2).unwrap();
            let four = *sample.iter().find(|segments| segments.count_ones() == 4).unwrap();
            let seven = *sample.iter().find(|segments| segments.count_ones() == 3).unwrap();
            let eight = *sample.iter().find(|segments| segments.count_ones() == 7).unwrap();

            // get the 6 segment digits
            let six_segs = sample.iter().filter(|segments| segments.count_ones() == 6).collect::<Vec<_>>();
            // the one that overlaps with only 1 segments with the 1 is the 6
            let six = **six_segs.iter().find(|&&&segments| (segments & one).count_ones() == 1).unwrap();
            // the one that overlaps with all 4 segments with the 4 is the 9
            let nine = **six_segs.iter().find(|&&&segments| (segments & four).count_ones() == 4).unwrap();
            // the one that is neither 6 or 9 must be the zero
            let zero = **six_segs.iter().find(|&&&segments| segments != nine && segments != six).unwrap();
            
            // get the 5 segment digits
            let five_segs = sample.iter().filter(|segments| segments.count_ones() == 5).collect::<Vec<_>>();
            // the one that overlaps with 2 segments with the 1 is the 3
            let three = **five_segs.iter().find(|&&segments| (segments & one).count_ones() == 2).unwrap();
            // the one that overlaps all 5 segments with the 6 is the 5
            let five = **five_segs.iter().find(|&&segments| (segments & six).count_ones() == 5).unwrap();
            // the one that is neither three or five must be the two
            let two = **five_segs.iter().find(|&&&segments| segments != three && segments != five).unwrap();

            vec![
                (zero, "0"),
                (one, "1"),
                (two, "2"),
                (three, "3"),
                (four, "4"),
                (five, "5"),
                (six, "6"),
                (seven, "7"),
                (eight, "8"),
                (nine, "9")].into_iter().collect::<HashMap<_,_>>()
            
        })
        .zip(
            include_str!("./input.txt").lines()
            .map(|line| line.split(" | ").nth(1).unwrap().split_ascii_whitespace().collect::<Vec<_>>())
        ).map(|(digits, output)| {
            output.iter()
            .map(|segments| segments_to_bits(segments))
            .map(move |segments| digits.get(&segments).unwrap().to_owned())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
        })
        //.collect::<Vec<_>>();
        .sum::<usize>();
    println!("Part 2: {:?}", result);
}

fn segments_to_bits(segments: &str) -> i32 {
    segments
        .chars()
        .fold(0_i32, |acc, part| {
            match part {
                'a' => acc | 0b1,
                'b' => acc | (0b1 << 1),
                'c' => acc | (0b1 << 2),
                'd' => acc | (0b1 << 3),
                'e' => acc | (0b1 << 4),
                'f' => acc | (0b1 << 5),
                'g' => acc | (0b1 << 6),
                _ => unimplemented!()
            }
        })
}
