use std::fs::read_to_string;

fn main() {
    let map = parse("input.txt");
    println!("part1 solution {}", count_visible_trees(&map));
    println!("part2 solution {}", find_max_scenic_score(&map));
}

fn count_visible_trees(map: &[Vec<u8>]) -> usize {
    let (size_x, size_y) = (map[0].len(), map.len());
    let items_on_edge = 2 * size_x + 2 * (size_y - 2);
    items_on_edge
        + (1..size_y - 1)
            .flat_map(|y| (1..size_x - 1).map(move |x| (y, x)))
            .filter(|&(y, x)| is_visible(map, y, x))
            .count()
}

fn is_visible(map: &[Vec<u8>], x: usize, y: usize) -> bool {
    for (y_diff, x_diff) in [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter() {
        let mut curr_x = x as isize + x_diff;
        let mut curr_y = y as isize + y_diff;
        loop {
            if !is_in_range(curr_x, curr_y, map[0].len(), map.len()) {
                return true;
            }
            if map[curr_y as usize][curr_x as usize] >= map[y][x] {
                break;
            }
            curr_y += y_diff;
            curr_x += x_diff;
        }
    }
    false
}

fn find_max_scenic_score(map: &[Vec<u8>]) -> usize {
    (1..map.len() - 1)
        .flat_map(|y| (1..map[0].len() - 1).map(move |x| (y, x)))
        .map(|(y, x)| calculate_single_score(map, y, x))
        .max()
        .unwrap()
}

fn calculate_single_score(map: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut values = [0; 4];
    for (idx, (y_diff, x_diff)) in [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter().enumerate() {
        values[idx] = {
            let mut curr_x = x as isize + x_diff;
            let mut curr_y = y as isize + y_diff;
            let mut count = 0;
            loop {
                if !is_in_range(curr_x, curr_y, map[0].len(), map.len()) {
                    break;
                }
                if map[curr_y as usize][curr_x as usize] < map[y][x] {
                    count += 1;
                }
                if map[curr_y as usize][curr_x as usize] >= map[y][x] {
                    count += 1;
                    break;
                }
                curr_y += y_diff;
                curr_x += x_diff;
            }
            count
        };
    }
    values.iter().product()
}
fn is_in_range(curr_x: isize, curr_y: isize, size_x: usize, size_y: usize) -> bool {
    curr_x >= 0 && curr_y >= 0 && curr_x < size_x as isize && curr_y < size_y as isize
}

fn parse(filename: &str) -> Vec<Vec<u8>> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| {
            line.split_terminator("")
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_visible_trees, find_max_scenic_score, parse};

    #[test]
    fn part1_test() {
        let map = parse("test-input.txt");
        assert_eq!(count_visible_trees(&map), 21)
    }

    #[test]
    fn part2_test() {
        let map = parse("test-input.txt");
        assert_eq!(find_max_scenic_score(&map), 8)
    }
}
