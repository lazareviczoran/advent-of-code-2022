use std::fs::read_to_string;

fn main() {
    let values = parse("input.txt");
    println!("part1 solution {}", sum_groove_coords(&values));
    println!("part2 solution {}", sum_groove_coords2(&values));
}

#[derive(Debug, Clone, Copy)]
struct Item {
    pos: usize,
    next: usize,
    prev: usize,
}

fn sum_groove_coords(initial_state: &[isize]) -> isize {
    let indices = shuffle(initial_state, 1);

    let mut current = initial_state.iter().position(|&v| v == 0).unwrap();
    let interesting_values = [1_000, 2_000, 3_000];
    let mut sum = 0;
    for i in 0..=3_000 {
        if interesting_values.contains(&i) {
            sum += initial_state[current];
        }
        current = indices[current].next;
    }

    sum
}

fn sum_groove_coords2(initial_state: &[isize]) -> isize {
    let initial_state = initial_state
        .iter()
        .map(|v| v * 811589153)
        .collect::<Vec<_>>();
    let indices = shuffle(&initial_state, 10);

    let mut current = initial_state.iter().position(|&v| v == 0).unwrap();
    let interesting_values = [1_000, 2_000, 3_000];
    let mut sum = 0;
    for i in 0..=3_000 {
        if interesting_values.contains(&i) {
            sum += initial_state[current];
        }
        current = indices[current].next;
    }

    sum
}

fn shuffle(initial_state: &[isize], n: usize) -> Vec<Item> {
    let values = initial_state.to_vec();
    let mut indices = values
        .iter()
        .enumerate()
        .map(|(idx, _)| Item {
            pos: idx,
            prev: if idx == 0 { values.len() - 1 } else { idx - 1 },
            next: (idx + 1) % values.len(),
        })
        .collect::<Vec<_>>();
    for _ in 0..n {
        for (curr_pos, value) in initial_state.iter().enumerate() {
            let mut i = *value % (initial_state.len() as isize - 1);
            while i != 0 {
                let current = indices[curr_pos];
                let (next, next2, prev) = if i < 0 {
                    (current.prev, indices[current.prev].prev, current.next)
                } else {
                    (current.next, indices[current.next].next, current.prev)
                };
                // swap positions
                (indices[curr_pos].pos, indices[next].pos) =
                    (indices[next].pos, indices[curr_pos].pos);
                if i < 0 {
                    (
                        indices[curr_pos].prev,
                        indices[curr_pos].next,
                        indices[next].prev,
                        indices[next].next,
                    ) = (indices[next].prev, next, curr_pos, indices[curr_pos].next);
                    indices[prev].prev = next;
                    indices[next2].next = curr_pos;
                } else {
                    (
                        indices[curr_pos].prev,
                        indices[curr_pos].next,
                        indices[next].prev,
                        indices[next].next,
                    ) = (next, indices[next].next, indices[curr_pos].prev, curr_pos);
                    indices[prev].next = next;
                    indices[next2].prev = curr_pos;
                }
                i -= i.signum();
            }
        }
    }
    indices
}

#[allow(dead_code)]
fn print_state(indices: &[(usize, usize, usize)], initial_state: &[isize]) {
    let mut current = initial_state.iter().position(|&v| v == 0).unwrap();
    let mut s = String::new();
    for _ in 0..initial_state.len() {
        s.push_str(&format!("{}  ", initial_state[current]));
        current = indices[current].2;
    }
    println!("{}", s);
}

fn parse(filename: &str) -> Vec<isize> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let values = parse("test-input.txt");
        assert_eq!(sum_groove_coords(&values), 3);
    }

    #[test]
    fn part2_test() {
        let values = parse("test-input.txt");
        assert_eq!(sum_groove_coords2(&values), 1623178306);
    }
}
