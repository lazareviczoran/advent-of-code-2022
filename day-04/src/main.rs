use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let ranges = parse("input.txt");
    println!(
        "part1 solution: {}",
        count_range_overlaps(&ranges, full_overlap)
    );
    println!(
        "part1 solution: {}",
        count_range_overlaps(&ranges, partial_overlap)
    );
}

fn count_range_overlaps<F>(
    ranges: &[(RangeInclusive<u64>, RangeInclusive<u64>)],
    filter_fn: F,
) -> usize
where
    F: Fn(&&(RangeInclusive<u64>, RangeInclusive<u64>)) -> bool,
{
    ranges.iter().filter(filter_fn).count()
}

fn full_overlap((a, b): &&(RangeInclusive<u64>, RangeInclusive<u64>)) -> bool {
    a.start() <= b.start() && a.end() >= b.end() || b.start() <= a.start() && b.end() >= a.end()
}

fn partial_overlap((a, b): &&(RangeInclusive<u64>, RangeInclusive<u64>)) -> bool {
    a.start() <= b.start() && a.end() >= b.start() || b.start() <= a.start() && b.end() >= a.start()
}

fn parse(filename: &str) -> Vec<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_once(',')?;
            let (first_from, first_to) = first.split_once('-')?;
            let (second_from, second_to) = second.split_once('-')?;
            Some((
                RangeInclusive::new(
                    first_from.parse::<u64>().ok()?,
                    first_to.parse::<u64>().ok()?,
                ),
                RangeInclusive::new(
                    second_from.parse::<u64>().ok()?,
                    second_to.parse::<u64>().ok()?,
                ),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_range_overlaps, full_overlap, parse, partial_overlap};

    #[test]
    fn part1_test() {
        let ranges = parse("test-input.txt");
        assert_eq!(count_range_overlaps(&ranges, full_overlap), 2)
    }

    #[test]
    fn part2_test() {
        let ranges = parse("test-input.txt");
        assert_eq!(count_range_overlaps(&ranges, partial_overlap), 4)
    }
}
