use std::fs::read_to_string;

fn main() {
    let actions = parse("input.txt");
    println!(
        "part1 solution: {}",
        actions
            .iter()
            .map(|(opp_action, my_action)| my_action.calculate_outcome(opp_action))
            .sum::<usize>()
    );
    println!(
        "part2 solution: {}",
        actions
            .iter()
            .map(|(opp_action, my_action)| Outcome::from(my_action).calculate_outcome(opp_action))
            .sum::<usize>()
    );
}

#[derive(PartialEq, Eq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}
impl From<&str> for Action {
    fn from(action: &str) -> Self {
        match action {
            "A" | "X" => Action::Rock,
            "B" | "Y" => Action::Paper,
            "C" | "Z" => Action::Scissors,
            _ => panic!("Invalid action"),
        }
    }
}
impl Action {
    fn get_value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn calculate_outcome(&self, action: &Action) -> usize {
        self.get_value()
            + match (action, self) {
                (Action::Paper, Self::Rock)
                | (Action::Scissors, Self::Paper)
                | (Action::Rock, Self::Scissors) => 0,
                (Action::Rock, Action::Rock)
                | (Action::Paper, Action::Paper)
                | (Action::Scissors, Action::Scissors) => 3,
                (Action::Rock, Self::Paper)
                | (Action::Paper, Self::Scissors)
                | (Action::Scissors, Self::Rock) => 6,
            }
    }
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}
impl From<&Action> for Outcome {
    fn from(outcome: &Action) -> Self {
        match outcome {
            Action::Rock => Self::Lose,
            Action::Paper => Self::Draw,
            Action::Scissors => Self::Win,
        }
    }
}
impl Outcome {
    fn get_value(&self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn calculate_outcome(&self, action: &Action) -> usize {
        self.get_value()
            + match (action, self) {
                (a, Self::Draw) => a.get_value(),
                (a, Self::Lose) => match a {
                    Action::Rock => Action::Scissors,
                    Action::Paper => Action::Rock,
                    Action::Scissors => Action::Paper,
                }
                .get_value(),
                (a, Self::Win) => match a {
                    Action::Rock => Action::Paper,
                    Action::Paper => Action::Scissors,
                    Action::Scissors => Action::Rock,
                }
                .get_value(),
            }
    }
}

fn parse(filename: &str) -> Vec<(Action, Action)> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| {
            let (action1, action2) = l.split_once(' ').unwrap();
            (action1.into(), action2.into())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse, Outcome};

    #[test]
    fn part1_test() {
        let actions = parse("test-input.txt");
        assert_eq!(
            actions
                .iter()
                .map(|(opp_action, my_action)| my_action.calculate_outcome(opp_action))
                .sum::<usize>(),
            15
        )
    }

    #[test]
    fn part2_test() {
        let actions = parse("test-input.txt");
        assert_eq!(
            actions
                .iter()
                .map(|(opp_action, my_action)| Outcome::from(my_action)
                    .calculate_outcome(opp_action))
                .sum::<usize>(),
            12
        )
    }
}
