use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let moves = parse("input.txt");
    println!("part1 solution {}", count_visited_for_tail(&moves));
    println!("part2 solution {}", count_visited_for_tail_2(&moves));
}

fn count_visited_for_tail_2(moves: &[Move]) -> usize {
    let mut knots: [(isize, isize); 10] = [(0, 0); 10];
    let mut tail_visited = HashSet::from([(knots[0])]);
    for m in moves {
        let (head_diff, repeat) = match m {
            Move::Right(steps) => ((1, 0), *steps),
            Move::Left(steps) => ((-1, 0), *steps),
            Move::Up(steps) => ((0, 1), *steps),
            Move::Down(steps) => ((0, -1), *steps),
        };
        for i in 1..=repeat.unsigned_abs() {
            for j in 0..knots.len() - 1 {
                let prev_idx = j;
                let current_tail_idx = 1 + j;
                (knots[prev_idx], knots[current_tail_idx]) = make_move(
                    knots[prev_idx],
                    knots[current_tail_idx],
                    head_diff,
                    &mut tail_visited,
                    prev_idx == 0,
                    current_tail_idx == 9,
                );
            }
        }
    }
    tail_visited.len()
}

fn count_visited_for_tail(moves: &[Move]) -> usize {
    let mut curr_head: (isize, isize) = (0, 0);
    let mut curr_tail: (isize, isize) = (0, 0);
    let mut tail_visited = HashSet::from([curr_tail]);
    for m in moves {
        let (head_diff, repeat) = match m {
            Move::Right(steps) => ((1, 0), *steps),
            Move::Left(steps) => ((-1, 0), *steps),
            Move::Up(steps) => ((0, 1), *steps),
            Move::Down(steps) => ((0, -1), *steps),
        };
        for _ in 0..repeat {
            (curr_head, curr_tail) = make_move(
                curr_head,
                curr_tail,
                head_diff,
                &mut tail_visited,
                true,
                true,
            );
        }
    }
    tail_visited.len()
}

fn make_move(
    head: (isize, isize),
    tail: (isize, isize),
    diff: (isize, isize),
    tail_visited: &mut HashSet<(isize, isize)>,
    is_head: bool,
    mark_visited: bool,
) -> ((isize, isize), (isize, isize)) {
    let prev_head = head;
    let (new_head, mut new_tail) = if is_head {
        ((prev_head.0 + diff.0, prev_head.1 + diff.1), tail)
    } else {
        (prev_head, tail)
    };
    let (x_diff, y_diff) = ((new_head.0 - tail.0), (new_head.1 - tail.1));
    if x_diff.abs() == 2 || y_diff.abs() == 2 {
        if x_diff.abs() > 0 && y_diff.abs() > 0 {
            if x_diff.abs() == 2 || y_diff.abs() == 2 {
                new_tail.1 += y_diff.signum();
                new_tail.0 += x_diff.signum();
            }
        } else if x_diff == 0 {
            new_tail.1 += y_diff.signum();
        } else if y_diff == 0 {
            new_tail.0 += x_diff.signum();
        }
    }
    if mark_visited {
        tail_visited.insert(new_tail);
    }
    (new_head, new_tail)
}

#[derive(Debug)]
enum Move {
    Right(isize),
    Left(isize),
    Up(isize),
    Down(isize),
}

fn parse(filename: &str) -> Vec<Move> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .filter_map(|line| {
            let (dir, steps) = line.split_once(' ')?;
            Some(match dir {
                "R" => Move::Right(steps.parse().ok()?),
                "L" => Move::Left(steps.parse().ok()?),
                "U" => Move::Up(steps.parse().ok()?),
                "D" => Move::Down(steps.parse().ok()?),
                _ => unreachable!(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_visited_for_tail, count_visited_for_tail_2, parse};

    #[test]
    fn part1_test() {
        let moves = parse("test-input.txt");
        assert_eq!(count_visited_for_tail(&moves), 13);
    }

    #[test]
    fn part2_test1() {
        let moves = parse("test-input.txt");
        assert_eq!(count_visited_for_tail_2(&moves), 1);
        // assert_eq!(2, 1);
    }

    #[test]
    fn part2_test2() {
        let moves = parse("test-input2.txt");
        assert_eq!(count_visited_for_tail_2(&moves), 36);
    }
}
