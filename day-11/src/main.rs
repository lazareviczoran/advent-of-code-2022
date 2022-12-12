use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let mut game = parse("input.txt");
    game.play_n_rounds(20, true);
    println!("part1 solution {}", game.score());

    let mut game = parse("input.txt");
    game.play_n_rounds(10000, false);
    println!("part2 solution {}", game.score());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    monkeys: Vec<Monkey>,
}
impl Game {
    fn play_n_rounds(&mut self, n: usize, divide_by_3: bool) {
        let prime_product = self.monkeys.iter().map(|m| m.divisible_by).product();
        for _ in 0..n {
            for m_idx in 0..self.monkeys.len() {
                let items = self.monkeys[m_idx].inspect(divide_by_3, prime_product);
                for (target_monkey, item) in items {
                    self.monkeys[target_monkey].items.push_back(item);
                }
            }
        }
    }

    fn score(&self) -> usize {
        let mut monkeys = self.monkeys.clone();
        monkeys.sort_by_key(|x| x.inspected_items);

        monkeys
            .into_iter()
            .rev()
            .take(2)
            .map(|x| x.inspected_items)
            .product()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Monkey {
    items: VecDeque<isize>,
    operation: Op,
    divisible_by: isize,
    true_target: usize,
    false_target: usize,
    inspected_items: usize,
}
impl Monkey {
    fn inspect(&mut self, divide_by_3: bool, prime_product: isize) -> Vec<(usize, isize)> {
        let mut result = Vec::new();
        while let Some(mut item) = self.items.pop_front() {
            match &self.operation {
                Op::Add(value) => item += value,
                Op::Multiply(value) => item *= value,
                Op::Square => item *= item,
            }
            match divide_by_3 {
                true => item /= 3,
                false => item %= prime_product,
            }
            let target_monkey = match item % self.divisible_by {
                0 => self.true_target,
                _ => self.false_target,
            };
            self.inspected_items += 1;
            result.push((target_monkey, item));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Op {
    Add(isize),
    Multiply(isize),
    Square,
}

fn parse(filename: &str) -> Game {
    let monkeys = read_to_string(filename)
        .expect("failed to read file")
        .split_terminator("\n\n")
        .filter_map(|monkey| {
            let lines = monkey.lines().collect::<Vec<_>>();
            let items = lines[1]
                .strip_prefix("  Starting items: ")?
                .split_terminator(", ")
                .filter_map(|x| x.parse().ok())
                .collect();
            let (op, value) = lines[2]
                .strip_prefix("  Operation: new = old ")?
                .split_once(' ')?;
            let operation = match (op, value) {
                ("*", "old") => Op::Square,
                ("*", _) => Op::Multiply(value.parse().ok()?),
                ("+", _) => Op::Add(value.parse().ok()?),
                _ => unreachable!(),
            };
            let divisible_by = lines[3]
                .strip_prefix("  Test: divisible by ")?
                .parse()
                .ok()?;
            let true_target = lines[4]
                .strip_prefix("    If true: throw to monkey ")?
                .parse()
                .ok()?;
            let false_target = lines[5]
                .strip_prefix("    If false: throw to monkey ")?
                .parse()
                .ok()?;
            Some(Monkey {
                items,
                operation,
                divisible_by,
                true_target,
                false_target,
                inspected_items: 0,
            })
        })
        .collect();
    Game { monkeys }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn part1_test() {
        let mut world = parse("test-input.txt");
        world.play_n_rounds(20, true);
        assert_eq!(world.score(), 10605);
    }

    #[test]
    fn part2_test() {
        let mut world = parse("test-input.txt");
        world.play_n_rounds(10000, false);
        assert_eq!(world.score(), 2713310158);
    }
}
