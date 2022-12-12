use std::{cmp::Ordering, collections::BinaryHeap, fs::read_to_string};

fn main() {
    let hill_map = parse("input.txt");
    println!(
        "part1 solution {:?}",
        shortest_path(&hill_map.map, hill_map.start, hill_map.top)
    );

    println!(
        "part2 solution {:?}",
        shortest_path_for_all_starts(&hill_map.map, hill_map.top)
    );
}

#[derive(Debug)]
struct Node {
    name: char,
    neighbours: Vec<((usize, usize), usize)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path_for_all_starts(map: &Vec<Vec<Node>>, target: (usize, usize)) -> Option<usize> {
    find_all_starting_positions(map, &['S', 'a'])
        .into_iter()
        .filter_map(|start| shortest_path(map, start, target))
        .min()
}

fn shortest_path(
    map: &Vec<Vec<Node>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];

    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        for &(next_pos, next_cost) in &map[position.0][position.1].neighbours {
            let next = State {
                cost: cost + next_cost,
                position: next_pos,
            };

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    None
}

fn find_position(map: &[Vec<char>], target: char) -> (usize, usize) {
    let mut curr = (0, 0);
    while map[curr.0][curr.1] != target {
        if curr.1 < map[0].len() - 1 {
            curr.1 += 1;
        } else {
            curr.0 += 1;
            curr.1 = 0;
        }
    }
    curr
}

fn find_all_starting_positions(map: &[Vec<Node>], target: &[char]) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if target.contains(&map[y][x].name) {
                positions.push((y, x));
            }
        }
    }
    positions
}

fn find_neighbours(map: &[Vec<char>], (y, x): (usize, usize)) -> Vec<((usize, usize), usize)> {
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .into_iter()
        .filter_map(|diff| {
            let next_pos = (y as isize + diff.0, x as isize + diff.1);
            if next_pos.0 >= 0
                && next_pos.0 < (map.len() as isize)
                && next_pos.1 >= 0
                && next_pos.1 < (map[0].len() as isize)
                && (map[y][x] != 'S'
                    && (map[next_pos.0 as usize][next_pos.1 as usize] == 'E' && map[y][x] == 'z'
                        || map[next_pos.0 as usize][next_pos.1 as usize] != 'E'
                            && map[next_pos.0 as usize][next_pos.1 as usize] as isize
                                - map[y][x] as isize
                                <= 1)
                    || map[y][x] == 'S' && map[next_pos.0 as usize][next_pos.1 as usize] == 'a')
            {
                Some(((next_pos.0 as usize, next_pos.1 as usize), 1))
            } else {
                None
            }
        })
        .collect()
}

struct HillMap {
    map: Vec<Vec<Node>>,
    start: (usize, usize),
    top: (usize, usize),
}

fn parse(filename: &str) -> HillMap {
    let map = read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| {
            line.split_terminator("")
                .filter_map(|ch| ch.chars().next())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let start = find_position(&map, 'S');
    let end = find_position(&map, 'E');

    let nodes_map = (0..map.len())
        .map(|y| {
            (0..map[0].len())
                .map(|x| Node {
                    name: map[y][x],
                    neighbours: find_neighbours(&map, (y, x)),
                })
                .collect()
        })
        .collect();
    HillMap {
        map: nodes_map,
        start,
        top: end,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, shortest_path, shortest_path_for_all_starts};

    #[test]
    fn part1_test() {
        let hill_map = parse("test-input.txt");
        assert_eq!(
            shortest_path(&hill_map.map, hill_map.start, hill_map.top),
            Some(31)
        );
    }

    #[test]
    fn part2_test() {
        let hill_map = parse("test-input.txt");
        assert_eq!(
            shortest_path_for_all_starts(&hill_map.map, hill_map.top),
            Some(29)
        );
    }
}
