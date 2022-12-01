use aoc_lib::util::transpose;

type Board = Vec<Vec<i32>>;

pub fn part1(input: String) -> i32 {
    let mut input = input.split("\n\n");
    let nums: Vec<i32> = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = input
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    for num in nums.into_iter() {
        boards = boards
            .into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|n| {
                                if n == num {
                                    return -1;
                                }
                                n
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        for board in boards.iter() {
            if winning_board(board) || winning_board(&transpose(board)) {
                return board.iter().flatten().filter(|x| **x != -1).sum::<i32>() * num;
            }
        }
        println!("");
    }
    0
}

fn winning_board(board: &Board) -> bool {
    board.iter().any(|row| row.iter().all(|x| *x == -1))
}

pub fn part2(input: String) -> i32 {
    let mut input = input.split("\n\n");
    let nums: Vec<i32> = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = input
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut last_board: Board = Vec::new();
    let mut q = 0;
    for num in nums.into_iter() {
        boards = boards
            .into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|n| {
                                if n == num {
                                    return -1;
                                }
                                n
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for board in boards.iter() {
            if winning_board(board) || winning_board(&transpose(board)) {
                last_board = board.clone();
                q = num;
            }
        }
        boards = boards
            .into_iter()
            .filter(|board| !winning_board(board) && !winning_board(&transpose(board)))
            .collect();
    }
    last_board
        .iter()
        .flatten()
        .filter(|x| **x != -1)
        .sum::<i32>()
        * q
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

    #[test]
    fn test_part1() {
        run_test!(part1);
    }

    #[test]
    fn run_part1() {
        run_real!(part1);
    }

    #[test]
    fn test_part2() {
        run_test!(part2);
    }

    #[test]
    fn run_part2() {
        run_real!(part2);
    }
}
