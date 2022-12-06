use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let chars = parse("input.txt");
    println!("part1 solution {}", find_marker(&chars, 4));
    println!("part2 solution {}", find_marker(&chars, 14));
}

fn find_marker(chars: &[char], window_size: usize) -> usize {
    chars
        .windows(window_size)
        .position(|window| HashSet::<&char>::from_iter(window.iter()).len() == window_size)
        .unwrap()
        + window_size
}

fn parse(filename: &str) -> Vec<char> {
    read_to_string(filename)
        .expect("failed to read file")
        .chars()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::find_marker;

    fn to_vec(input: &str) -> Vec<char> {
        input.chars().collect()
    }

    #[test]
    fn part1_test() {
        assert_eq!(find_marker(&to_vec("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 4), 7);
        assert_eq!(find_marker(&to_vec("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4), 5);
        assert_eq!(find_marker(&to_vec("nppdvjthqldpwncqszvftbrmjlhg"), 4), 6);
        assert_eq!(
            find_marker(&to_vec("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4),
            10
        );
        assert_eq!(
            find_marker(&to_vec("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 4),
            11
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            find_marker(&to_vec("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14),
            19
        );
        assert_eq!(find_marker(&to_vec("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14), 23);
        assert_eq!(find_marker(&to_vec("nppdvjthqldpwncqszvftbrmjlhg"), 14), 23);
        assert_eq!(
            find_marker(&to_vec("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 14),
            29
        );
        assert_eq!(
            find_marker(&to_vec("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 14),
            26
        );
    }
}
