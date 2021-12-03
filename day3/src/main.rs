fn main() {
    part1();
    part2();
}

fn part1() {
    let line_count = include_str!("./input.txt").lines().count();
    let input = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let gamma = get_bit_counts(&input)
        .into_iter()
        .rev()
        .enumerate()
        .fold(0u32, |acc , (pos, bit_count)|  {
            if bit_count as usize > (line_count >> 1) { 
                acc + (1 << pos)
            } else { 
                acc 
            }
        });

    let epsilon = !gamma & ((1 << 12) - 1);

    println!("Part1: {}", gamma * epsilon);
}

fn part2() {
    let ogr_values = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let ogr_value = (0..12)
        .fold(ogr_values, |acc: Vec<_>, pos| {
            if acc.len() == 1 {
                acc
            } else {
                let bit_counts = get_bit_counts(&acc);
                dbg!(&bit_counts);
                let values = acc.iter()
                    .filter(|&value| {
                        println!("{} / {} / {}", value, pos, bit_counts[pos]);
                        if bit_counts[pos]*2 >= acc.len() as u32 {
                            value.chars().nth(pos).unwrap() == '1'
                        } else {
                            value.chars().nth(pos).unwrap() == '0'
                        }
                    })
                    .map(|v| *v)
                    .collect::<Vec<_>>();
                dbg!(&values);

                values
            }
        })[0]
        .chars()
        .rev()
        .enumerate()
        .fold(0u32, |acc , (pos, bit)|  {
            if bit == '1' {
                acc + (1 << pos)
            } else { 
                acc 
            }
        });
    dbg!(&ogr_value);


    let csr_values = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let csr_value = (0..12)
        .fold(csr_values, |acc: Vec<_>, pos| {
            if acc.len() == 1 {
                acc
            } else {
                let bit_counts = get_bit_counts(&acc);
                dbg!(&bit_counts);
                let values = acc.iter()
                    .filter(|&value| {
                        println!("{} / {} / {}", value, pos, bit_counts[pos]);
                        if bit_counts[pos]*2 >= acc.len() as u32 {
                            value.chars().nth(pos).unwrap() == '0'
                        } else {
                            value.chars().nth(pos).unwrap() == '1'
                        }
                    })
                    .map(|v| *v)
                    .collect::<Vec<_>>();
                dbg!(&values);

                values
            }
        })[0]
        .chars()
        .rev()
        .enumerate()
        .fold(0u32, |acc , (pos, bit)|  {
            if bit == '1' {
                acc + (1 << pos)
            } else { 
                acc 
            }
        });
    dbg!(&csr_value);

    println!("Part2: {}", ogr_value * csr_value);
}

fn get_bit_counts(values: &[&str]) -> Vec<u32> {
    values.into_iter()
        .fold(vec![0,0,0,0,0,0,0,0,0,0,0,0], |acc, bits| {
            acc.into_iter().zip(
                bits.chars().map(|c| match c { '0' => 0, '1' => 1, _ => unimplemented!()}).into_iter()
            )
            .map(|(last, current)| last + current)
            .collect::<Vec<u32>>()
        })
}