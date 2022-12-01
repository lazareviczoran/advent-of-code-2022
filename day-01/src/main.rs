use std::{collections::BTreeSet, fs::read_to_string};

fn main() {
    let sums = parse("input.txt");
    println!("part1 solution: {:?}", get_top_n_sum(&sums, 1));
    println!("part2 solution: {}", get_top_n_sum(&sums, 3));
}

fn get_top_n_sum(sums: &BTreeSet<isize>, n: usize) -> isize {
    sums.iter().rev().take(n).sum()
}

fn parse(filename: &str) -> BTreeSet<isize> {
    read_to_string(filename)
        .expect("failed to read file")
        .split_terminator("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<isize>().unwrap()).sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{get_top_n_sum, parse};

    #[test]
    fn part1_test() {
        let sums = parse("test-input.txt");
        assert_eq!(24000, get_top_n_sum(&sums, 1));
    }

    #[test]
    fn part2_test() {
        let sums = parse("test-input.txt");
        assert_eq!(45000, get_top_n_sum(&sums, 3));
    }
}
