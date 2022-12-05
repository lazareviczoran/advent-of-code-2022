use std::fs::read_to_string;

fn main() {
    let (mut crates, moves) = parse("input.txt");
    crates.apply_moves_v9000(&moves);
    println!("part1 solution {}", crates.get_message());
    crates.reset();
    crates.apply_moves_v9001(&moves);
    println!("part2 solution {}", crates.get_message());
}

#[derive(Debug)]
struct Move {
    num_of_crates_to_move: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Crates {
    state: Vec<Vec<char>>,
    initial_state: Vec<Vec<char>>,
}
impl Crates {
    fn apply_moves_v9000(&mut self, moves: &[Move]) {
        moves.iter().for_each(|m| {
            (0..m.num_of_crates_to_move).for_each(|_| {
                if let Some(ch) = self.state[m.from - 1].pop() {
                    self.state[m.to - 1].push(ch);
                }
            });
        })
    }

    fn apply_moves_v9001(&mut self, moves: &[Move]) {
        moves.iter().for_each(|m| {
            let lifted = (0..m.num_of_crates_to_move).fold(vec![], |mut acc, _| {
                acc.push(self.state[m.from - 1].pop().unwrap());
                acc
            });
            lifted.into_iter().rev().for_each(|ch| {
                self.state[m.to - 1].push(ch);
            });
        });
    }

    fn reset(&mut self) {
        self.state = self.initial_state.clone();
    }

    fn get_message(&self) -> String {
        self.state.iter().filter_map(|stack| stack.last()).collect()
    }
}
impl From<&str> for Crates {
    fn from(text: &str) -> Self {
        let lines = text.lines().collect::<Vec<_>>();
        let num_of_buckets = lines
            .last()
            .unwrap()
            .split_terminator(' ')
            .filter(|ch| !ch.is_empty())
            .count();
        let state =
            lines
                .iter()
                .rev()
                .skip(1)
                .fold(vec![vec![]; num_of_buckets], |mut acc, line| {
                    let mut chars_iter = line.chars().enumerate().peekable();
                    while chars_iter.peek().is_some() {
                        let (idx, first_char) = chars_iter.next().unwrap();
                        let mut chars_to_skip = 3;
                        if first_char == '[' {
                            let bucket = idx / 4;
                            let (_, item) = chars_iter.next().unwrap();
                            acc[bucket].push(item);
                            chars_to_skip -= 1;
                        }
                        (0..chars_to_skip).for_each(|_| {
                            chars_iter.next();
                        });
                    }
                    acc
                });
        let initial_state = state.clone();
        Self {
            state,
            initial_state,
        }
    }
}

fn parse(filename: &str) -> (Crates, Vec<Move>) {
    let content = read_to_string(filename).expect("failed to read file");
    let (crates_str, moves_str) = content.split_once("\n\n").unwrap();
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves_str
        .lines()
        .filter_map(|line| {
            let captures = re.captures_iter(line).next()?;
            let num_of_crates_to_move = captures[1].parse::<usize>().ok()?;
            let from = captures[2].parse::<usize>().ok()?;
            let to = captures[3].parse::<usize>().ok()?;

            Some(Move {
                num_of_crates_to_move,
                from,
                to,
            })
        })
        .collect();
    (Crates::from(crates_str), moves)
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn part1_test() {
        let (mut crates, moves) = parse("test-input.txt");
        crates.apply_moves_v9000(&moves);
        assert_eq!(crates.get_message(), "CMZ");
    }

    #[test]
    fn part2_test() {
        let (mut crates, moves) = parse("test-input.txt");
        crates.apply_moves_v9001(&moves);
        assert_eq!(crates.get_message(), "MCD");
    }
}
