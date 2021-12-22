fn main() {
    part1();
    part2();
}

fn apply_drawing(boards: &mut Vec<Vec<Option<i32>>>, draw: i32) {
    for board in boards.iter_mut() {
        if let Some(cell) = board.iter_mut().find(|cell| match cell {
            Some(num) if num == &draw => true,
            _ => false,
        }) {
            *cell = match cell {
                Some(_) => None,
                _ => unimplemented!(),
            }
        }
    }
}

fn part1() {
    let input = include_str!("./input.txt");
    let drawing = input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();

    let input = input.lines().skip(2).collect::<Vec<_>>();
    let mut boards = input
        .chunks(6)
        .map(|rows| {
            rows.iter()
                .filter(|&&c| c != "")
                .map(|cols| {
                    cols.split_ascii_whitespace()
                        .map(|cell| Some(cell.parse::<i32>().unwrap()))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut winner_board = None;
    let mut winner_drawing = 0;
    for draw in drawing {
        for board in boards.iter_mut() {
            if let Some(cell) = board.iter_mut().find(|cell| match cell {
                Some(num) if num == &draw => true,
                _ => false,
            }) {
                *cell = match cell {
                    Some(_) => None,
                    _ => unimplemented!(),
                }
            }
        }

        let wb = boards
            .iter()
            .filter(|board| {
                let row_full = board
                    .chunks_exact(5)
                    .filter(|v| v.iter().filter(|c| c.is_none()).count() == 5)
                    .count()
                    != 0;
                let column_full = (0..5).into_iter().fold(false, |acc, col| {
                    acc | (board
                        .iter()
                        .skip(col)
                        .step_by(5)
                        .filter(|c| c.is_none())
                        .count()
                        == 5)
                });

                row_full || column_full
            })
            .nth(0)
            .map(|b| b.clone());

        if let Some(wb) = wb {
            winner_board.replace(wb);
            winner_drawing = draw;
            break;
        }
    }

    if let Some(wb) = winner_board {
        let points = wb.iter().fold(
            0,
            |acc, cell| {
                if let Some(v) = cell {
                    acc + v
                } else {
                    acc
                }
            },
        ) * winner_drawing;

        println!("Part1: {}", points);
    }
}

fn part2() {
    let input = include_str!("./input.txt");
    let drawing = input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();

    let input = input.lines().skip(2).collect::<Vec<_>>();
    let mut boards = input
        .chunks(6)
        .map(|rows| {
            rows.iter()
                .filter(|&&c| c != "")
                .map(|cols| {
                    cols.split_ascii_whitespace()
                        .map(|cell| Some(cell.parse::<i32>().unwrap()))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut winner_board = None;
    let mut winner_drawing = 0;
    for draw in drawing {
        for board in boards.iter_mut() {
            if let Some(cell) = board.iter_mut().find(|cell| match cell {
                Some(num) if num == &draw => true,
                _ => false,
            }) {
                *cell = match cell {
                    Some(_) => None,
                    _ => unimplemented!(),
                }
            }
        }

        let open_boards = boards
            .iter()
            .filter(|board| {
                let row_full = board
                    .chunks_exact(5)
                    .filter(|v| v.iter().filter(|c| c.is_none()).count() == 5)
                    .count()
                    != 0;
                let column_full = (0..5).into_iter().fold(false, |acc, col| {
                    acc | (board
                        .iter()
                        .skip(col)
                        .step_by(5)
                        .filter(|c| c.is_none())
                        .count()
                        == 5)
                });

                !(row_full || column_full)
            })
            .map(|b| b.clone())
            .collect::<Vec<_>>();

        if boards.len() == 1 && open_boards.len() == 0 {
            if let Some(lwb) = boards.get(0).map(|b| b.clone()) {
                winner_board.replace(lwb);
                winner_drawing = draw;
                break;
            }
        } else {
            boards = open_boards;
        }
    }

    if let Some(wb) = winner_board {
        let points = wb.iter().fold(
            0,
            |acc, cell| {
                if let Some(v) = cell {
                    acc + v
                } else {
                    acc
                }
            },
        ) * winner_drawing;

        println!("Part2: {}", points);
    }
}
