use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let ranges = parse("input.txt");
    println!("part1 solution: {}", count_overlaps(&ranges, is_full));
    println!("part2 solution: {}", count_overlaps(&ranges, is_partial));
}

type RangePair = (RangeInclusive<u64>, RangeInclusive<u64>);
fn count_overlaps<F>(ranges: &[RangePair], filter_fn: F) -> usize
where
    F: Fn(&&RangePair) -> bool,
{
    ranges.iter().filter(filter_fn).count()
}

fn is_full((a, b): &&RangePair) -> bool {
    a.start() <= b.start() && a.end() >= b.end() || b.start() <= a.start() && b.end() >= a.end()
}

fn is_partial((a, b): &&RangePair) -> bool {
    a.start() <= b.start() && a.end() >= b.start() || b.start() <= a.start() && b.end() >= a.start()
}

fn parse(filename: &str) -> Vec<RangePair> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_once(',')?;
            let (first_from, first_to) = first.split_once('-')?;
            let (second_from, second_to) = second.split_once('-')?;
            Some((
                (first_from.parse::<u64>().ok()?..=first_to.parse::<u64>().ok()?),
                (second_from.parse::<u64>().ok()?..=second_to.parse::<u64>().ok()?),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_overlaps, is_full, is_partial, parse};

    #[test]
    fn part1_test() {
        let ranges = parse("test-input.txt");
        assert_eq!(count_overlaps(&ranges, is_full), 2)
    }

    #[test]
    fn part2_test() {
        let ranges = parse("test-input.txt");
        assert_eq!(count_overlaps(&ranges, is_partial), 4)
    }
}
