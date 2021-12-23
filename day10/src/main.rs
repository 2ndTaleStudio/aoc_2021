fn main() {
    part1();
    part2();
}

fn part1() {
    let result = include_str!("./input.txt")
        .lines()
        .fold(0, |acc, line| {
            if let Some(error) = line.chars().fold((vec![], None), |(mut chunks, error), c| {
                if error.is_some() {
                    (chunks, error)
                } else {
                    if is_open_char(&c) {
                        chunks.push(c);
                        (chunks, None)
                    } else {
                        if is_matching_close_char(&chunks[chunks.len()-1], &c) {
                            chunks.pop();
                            (chunks, None)
                        } else {
                            (chunks, Some(error_points(&c)))
                        }
                    }
                }
            }).1 {
                acc + error
            } else {
                acc
            }
        });
    println!("Part 1: {:?}", result);
}

fn part2() {
    let mut result: Vec<usize> = include_str!("./input.txt")
    .lines()
    .filter(|line| {
        line.chars().fold((vec![], None), |(mut chunks, error), c| {
            if error.is_some() {
                (chunks, error)
            } else {
                if is_open_char(&c) {
                    chunks.push(c);
                    (chunks, None)
                } else {
                    if is_matching_close_char(&chunks[chunks.len()-1], &c) {
                        chunks.pop();
                        (chunks, None)
                    } else {
                        (chunks, Some(error_points(&c)))
                    }
                }
            }
        }).1.is_none()
    })
    .map(|line| {
        line.chars().fold(vec![], |mut chunks, c| {
            if is_open_char(&c) {
                chunks.push(c);
                chunks
            } else {
                if is_matching_close_char(&chunks[chunks.len()-1], &c) {
                    chunks.pop();
                    chunks
                } else {
                    panic!() // should never happen!
                }
            }
        }).iter().rev().fold(0usize, |acc, c| {
            acc * 5usize + missing_points(&c) as usize
        })
    }).collect::<Vec<_>>();

    result.sort();

    println!("Part 2: {:?}", result[result.len() >> 1]);
}

fn is_open_char(c: &char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn is_matching_close_char(open_c: &char, c: &char) -> bool {
    match (open_c, c) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn error_points(c: &char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unimplemented!(),
    }
}

fn missing_points(c: &char) -> i32 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unimplemented!(),
    }
}