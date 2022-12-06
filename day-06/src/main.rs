use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let chars = parse("input.txt");
    println!("part1 solution {}", find_marker_score_n(&chars, 4));
    println!("part2 solution {}", find_marker_score_n(&chars, 14));
}

fn find_marker_score_n(chars: &[char], window_size: usize) -> usize {
    let mut items = HashMap::<char, usize>::new();
    for (idx, current_window) in chars.windows(window_size).enumerate() {
        if items.is_empty() {
            current_window.iter().for_each(|ch| {
                *items.entry(*ch).or_insert(0) += 1;
            });
        } else {
            *items.entry(current_window[window_size - 1]).or_insert(0) += 1;
        }
        if items.iter().any(|(_k, v)| *v > 1) {
            let current_value = *items.get(&current_window[0]).unwrap();
            if current_value > 1 {
                items.insert(current_window[0], current_value - 1);
            } else {
                items.remove(&current_window[0]);
            }
            continue;
        }
        return idx + window_size;
    }

    unreachable!()
}

fn parse(filename: &str) -> Vec<char> {
    read_to_string(filename)
        .expect("failed to read file")
        .chars()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::find_marker_score_n;

    fn chars_to_vec(input: &str) -> Vec<char> {
        input.chars().collect()
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            find_marker_score_n(&chars_to_vec("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 4),
            7
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4),
            5
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("nppdvjthqldpwncqszvftbrmjlhg"), 4),
            6
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4),
            10
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 4),
            11
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            find_marker_score_n(&chars_to_vec("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14),
            19
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14),
            23
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("nppdvjthqldpwncqszvftbrmjlhg"), 14),
            23
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 14),
            29
        );
        assert_eq!(
            find_marker_score_n(&chars_to_vec("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 14),
            26
        );
    }
}
