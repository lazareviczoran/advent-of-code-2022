use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let rucksacks = parse("input.txt");
    println!("part1 solution {:?}", split_and_find(&rucksacks));
    println!("part2 solution {:?}", group_and_find(&rucksacks));
}

fn split_and_find(input: &[String]) -> usize {
    let rucksacks_iter = input.iter().map(|rucksack| {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        &HashSet::from_iter(first.chars()) & &HashSet::from_iter(second.chars())
    });
    calculate_score(rucksacks_iter)
}

fn group_and_find(input: &[String]) -> usize {
    let rucksacks_iter = input
        .chunks_exact(3)
        .map(|chunk| chunk.iter().map(|item| HashSet::from_iter(item.chars())))
        .filter_map(|mut chunk| {
            let (item1, item2, item3) = (chunk.next()?, chunk.next()?, chunk.next()?);
            Some(&(&item1 & &item2) & &item3)
        });
    calculate_score(rucksacks_iter)
}

fn calculate_score<I>(items_iter: I) -> usize
where
    I: Iterator<Item = HashSet<char>>,
{
    items_iter
        .filter_map(|set| Some(get_value(*set.iter().next()?)))
        .sum()
}

fn get_value(letter: char) -> usize {
    match letter {
        'A'..='Z' => letter as usize - 'A' as usize + 27,
        'a'..='z' => letter as usize - 'a' as usize + 1,
        _ => 0,
    }
}

fn parse(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.into())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::{group_and_find, parse, split_and_find};

    #[test]
    fn part1_test() {
        let rucksacks = parse("test-input.txt");
        assert_eq!(split_and_find(&rucksacks), 157)
    }

    #[test]
    fn part2_test() {
        let rucksacks = parse("test-input.txt");
        assert_eq!(group_and_find(&rucksacks), 70)
    }
}
