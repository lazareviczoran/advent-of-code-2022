use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let monkeys = parse("input.txt");
    println!(
        "part1 solution {:?}",
        eval_monkey_exprs(&monkeys, monkeys.get(&"root".to_string()).unwrap(), false)
    );
    let pt2 = find_number_to_yell(&monkeys, "root", None);
    println!("part2 solution {}", pt2);
}

#[derive(Debug, Clone)]
enum Value {
    Number(isize),
    Expr { op: Op, v1: String, v2: String },
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn find_number_to_yell(
    monkey_exprs: &HashMap<String, Value>,
    item: &str,
    target: Option<isize>,
) -> isize {
    let root = monkey_exprs.get(item).unwrap();
    let (left, right, op) = match root {
        Value::Expr { v1, v2, op } => (v1, v2, op),
        _ => unreachable!(),
    };
    let left_val;
    let right_val;
    if left == "humn" {
        right_val = eval_monkey_exprs(monkey_exprs, monkey_exprs.get(right).unwrap(), true);
        left_val = None;
    } else if right == "humn" {
        left_val = eval_monkey_exprs(monkey_exprs, monkey_exprs.get(left).unwrap(), true);
        right_val = None;
    } else {
        left_val = eval_monkey_exprs(monkey_exprs, monkey_exprs.get(left).unwrap(), true);
        right_val = eval_monkey_exprs(monkey_exprs, monkey_exprs.get(right).unwrap(), true);
    }
    let calculate_remaining: Box<dyn Fn(isize) -> isize> = match (item, op) {
        (current_item, _) if current_item == "root" => Box::new(|value| value),
        (_, Op::Add) => Box::new(|value| target.unwrap_or_default() - value),
        (_, Op::Sub) => Box::new(|value| {
            if left_val.is_some() {
                value - target.unwrap_or_default()
            } else {
                target.unwrap_or_default() + value
            }
        }),
        (_, Op::Mul) => Box::new(|value| target.unwrap_or_default() / value),
        (_, Op::Div) => Box::new(|value| target.unwrap_or_default() * value),
    };
    match (left_val, right_val) {
        (Some(left_res), None) if right != "humn" => {
            find_number_to_yell(monkey_exprs, right, Some(calculate_remaining(left_res)))
        }
        (None, Some(right_res)) if left != "humn" => {
            find_number_to_yell(monkey_exprs, left, Some(calculate_remaining(right_res)))
        }
        (Some(left_res), None) if right == "humn" => calculate_remaining(left_res),
        (None, Some(right_res)) if left == "humn" => calculate_remaining(right_res),
        invalid => unreachable!("{invalid:?}"),
    }
}

fn eval_monkey_exprs(
    monkey_exprs: &HashMap<String, Value>,
    item: &Value,
    find_human_value: bool,
) -> Option<isize> {
    match item {
        Value::Number(n) => Some(*n),
        Value::Expr { op, v1, v2 } => {
            let (val1, val2) = (
                if find_human_value && v1 == "humn" {
                    None
                } else {
                    Some(eval_monkey_exprs(
                        monkey_exprs,
                        monkey_exprs.get(v1).unwrap(),
                        find_human_value,
                    )?)
                },
                if find_human_value && v2 == "humn" {
                    None
                } else {
                    Some(eval_monkey_exprs(
                        monkey_exprs,
                        monkey_exprs.get(v2).unwrap(),
                        find_human_value,
                    )?)
                },
            );
            Some(match op {
                Op::Add => val1? + val2?,
                Op::Sub => val1? - val2?,
                Op::Mul => val1? * val2?,
                Op::Div => val1? / val2?,
            })
        }
    }
}

fn parse(filename: &str) -> HashMap<String, Value> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| {
            let (monkey, value_str) = l.split_once(": ").unwrap();
            let value = match value_str.chars().next() {
                Some(x) if x.is_alphabetic() => {
                    let parts = value_str.splitn(3, ' ').collect::<Vec<_>>();
                    Value::Expr {
                        op: match parts[1] {
                            "+" => Op::Add,
                            "-" => Op::Sub,
                            "*" => Op::Mul,
                            "/" => Op::Div,
                            invalid => unreachable!("{invalid}"),
                        },
                        v1: parts[0].into(),
                        v2: parts[2].into(),
                    }
                }
                Some(x) if x.is_ascii_digit() => Value::Number(value_str.parse().unwrap()),
                _ => unreachable!(),
            };

            (monkey.into(), value)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let monkeys = parse("test-input.txt");
        assert_eq!(
            eval_monkey_exprs(&monkeys, monkeys.get(&"root".to_string()).unwrap(), false),
            Some(152)
        );
    }

    #[test]
    fn part2_test() {
        let monkeys = parse("test-input.txt");
        assert_eq!(find_number_to_yell(&monkeys, "root", None), 301);
    }
}
