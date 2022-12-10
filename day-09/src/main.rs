use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let moves = parse("input.txt");
    println!("part1 solution {}", count_visited_for_tail::<2>(&moves));
    println!("part2 solution {}", count_visited_for_tail::<10>(&moves));
}

fn count_visited_for_tail<const N: usize>(moves: &[Move]) -> usize {
    let mut knots: [(isize, isize); N] = [(0, 0); N];
    let mut tail_visited = HashSet::from([(knots[0])]);
    for m in moves {
        let (diff, repeat) = match m {
            Move::Right(steps) => ((1, 0), *steps),
            Move::Left(steps) => ((-1, 0), *steps),
            Move::Up(steps) => ((0, 1), *steps),
            Move::Down(steps) => ((0, -1), *steps),
        };
        for _ in 1..=repeat.unsigned_abs() {
            knots[0] = (knots[0].0 + diff.0, knots[0].1 + diff.1);
            for curr_idx in 0..knots.len() - 1 {
                let next_idx = curr_idx + 1;
                (knots[next_idx]) = move_tail(
                    knots[curr_idx],
                    knots[next_idx],
                    &mut tail_visited,
                    next_idx == knots.len() - 1,
                );
            }
        }
    }
    tail_visited.len()
}

fn move_tail(
    head: (isize, isize),
    tail: (isize, isize),
    tail_visited: &mut HashSet<(isize, isize)>,
    mark_visited: bool,
) -> (isize, isize) {
    let mut new_tail = tail;
    let (x_diff, y_diff) = ((head.0 - tail.0), (head.1 - tail.1));
    if x_diff.abs() == 2 || y_diff.abs() == 2 {
        if x_diff.abs() > 0 && y_diff.abs() > 0 {
            new_tail.1 += y_diff.signum();
            new_tail.0 += x_diff.signum();
        } else if x_diff == 0 {
            new_tail.1 += y_diff.signum();
        } else if y_diff == 0 {
            new_tail.0 += x_diff.signum();
        }
    }
    if mark_visited {
        tail_visited.insert(new_tail);
    }
    new_tail
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
    use crate::{count_visited_for_tail, parse};

    #[test]
    fn part1_test() {
        let moves = parse("test-input.txt");
        assert_eq!(count_visited_for_tail::<2>(&moves), 13);
    }

    #[test]
    fn part2_test1() {
        let moves = parse("test-input.txt");
        assert_eq!(count_visited_for_tail::<10>(&moves), 1);
    }

    #[test]
    fn part2_test2() {
        let moves = parse("test-input2.txt");
        assert_eq!(count_visited_for_tail::<10>(&moves), 36);
    }
}
