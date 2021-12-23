fn main() {
    part1();
    part2();
}

fn part1() {
    let hight_map = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = hight_map
        .iter()
        .enumerate()
        .map(|(row, data)| {
            data.iter().enumerate().fold(0, |acc, (col, cell)| {
                if hight_map[row.saturating_sub(1)..=usize::min(row + 1, hight_map.len() - 1)]
                    .iter()
                    .find(|data| {
                        data[col.saturating_sub(1)..=usize::min(col + 1, data.len() - 1)]
                            .iter()
                            .find(|&c| c < cell)
                            .is_some()
                    })
                    .is_none()
                {
                    acc + cell + 1
                } else {
                    acc
                }
            })
        })
        .sum::<u32>();

    println!("Part 1: {:?}", result);
}

fn part2() {
    let hight_map = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = hight_map
        .iter()
        .enumerate()
        .map(|(row, data)| {
            data.iter()
                .enumerate()
                .fold(vec![], |mut acc, (col, cell)| {
                    if hight_map[row.saturating_sub(1)..=usize::min(row + 1, hight_map.len() - 1)]
                        .iter()
                        .find(|data| {
                            data[col.saturating_sub(1)..=usize::min(col + 1, data.len() - 1)]
                                .iter()
                                .find(|&c| c < cell)
                                .is_some()
                        })
                        .is_none()
                    {
                        // collect the low-point positions for each row
                        acc.push((row, col));
                    };

                    acc
                })
        })
        .flatten()
        .collect::<Vec<_>>();

    // for each low-point calculate the basin size
    let mut basins = result
        .iter()
        .map(|(row, col)| {
            get_basin(
                &hight_map,
                &mut vec![],
                (*row, *col),
                (hight_map.len(), hight_map[0].len()),
            )
            .len()
        })
        .collect::<Vec<_>>();
    basins.sort_by(|a, b| b.cmp(a));

    println!("Part 2: {:?}", basins[0] * basins[1] * basins[2]);
}

// from the current position go to left, up, right, down as long as we do not find a '9' and add
// this position to the current basin and repeat for all found position
fn get_basin(
    map: &Vec<Vec<u32>>,
    visited: &mut Vec<(usize, usize)>,
    current: (usize, usize),
    size: (usize, usize),
) -> Vec<(usize, usize)> {
    let row_up = current.0.saturating_sub(1);
    let col_left = current.1.saturating_sub(1);
    let row_down = usize::min(current.0 + 1, size.0 - 1);
    let col_right = usize::min(current.1 + 1, size.1 - 1);

    let mut items = vec![];
    visited.push(current);

    // one up
    if row_up != current.0 && map[row_up][current.1] < 9 {
        if visited
            .iter()
            .find(|(r, c)| *r == row_up && *c == current.1)
            .is_none()
        {
            items.append(&mut get_basin(map, visited, (row_up, current.1), size));
        }
    }
    // one right
    if col_right != current.1 && map[current.0][col_right] < 9 {
        if visited
            .iter()
            .find(|(r, c)| *r == current.0 && *c == col_right)
            .is_none()
        {
            items.append(&mut get_basin(map, visited, (current.0, col_right), size));
        }
    }
    // one down
    if row_down != current.0 && map[row_down][current.1] < 9 {
        if visited
            .iter()
            .find(|(r, c)| *r == row_down && *c == current.1)
            .is_none()
        {
            items.append(&mut get_basin(map, visited, (row_down, current.1), size));
        }
    }
    // one left
    if col_left != current.1 && map[current.0][col_left] < 9 {
        if visited
            .iter()
            .find(|(r, c)| *r == current.0 && *c == col_left)
            .is_none()
        {
            items.append(&mut get_basin(map, visited, (current.0, col_left), size));
        }
    };

    items.push(current);
    items
}
