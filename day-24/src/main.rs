use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let mut maze = parse("input.txt");
    let p1 = maze.find_min_steps(maze.start, maze.target);
    println!("part1 solution {p1}");
    let p2 = maze.find_min_steps(maze.target, maze.start);
    let p3 = maze.find_min_steps(maze.start, maze.target);
    println!("part2 solution {}", p1 + p2 + p3);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Maze {
    map: Vec<Vec<char>>,
    width: isize,
    height: isize,
    start: (isize, isize),
    target: (isize, isize),
    blizzards: Vec<((isize, isize), char)>,
    steps: usize,
    applied_blizzards: isize,
}

impl Maze {
    fn find_min_steps(&mut self, start: (isize, isize), target: (isize, isize)) -> usize {
        let mut q = VecDeque::new();
        q.push_back((start, 0));
        let mut cache = HashSet::new();
        while let Some((curr_pos, steps)) = q.pop_front() {
            if curr_pos == target {
                self.applied_blizzards = 0;
                return steps;
            }
            if self.applied_blizzards < steps as isize {
                self.move_blizzards();
            }
            if cache.contains(&(curr_pos, self.blizzards.clone())) {
                continue;
            }
            cache.insert((curr_pos, self.blizzards.clone()));
            for diff in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
                let next_pos = (curr_pos.0 + diff.0, curr_pos.1 + diff.1);
                if next_pos.0 < 0
                    || next_pos.0 >= self.height
                    || next_pos.1 < 0
                    || next_pos.1 >= self.width
                    || self.blizzards.iter().any(|&(pos, _)| next_pos == pos)
                    || self.map[next_pos.0 as usize][next_pos.1 as usize] == '#'
                {
                    continue;
                }
                q.push_back((next_pos, steps + 1));
            }
            if curr_pos != (0, 0) && !self.blizzards.iter().any(|&(pos, _)| curr_pos == pos) {
                q.push_back((curr_pos, steps + 1));
            }
        }
        unreachable!()
    }

    fn move_blizzards(&mut self) {
        self.applied_blizzards += 1;
        self.blizzards = self
            .blizzards
            .clone()
            .into_iter()
            .map(|((y, x), ch)| {
                let mut next_pos = match ch {
                    '<' => (y, x - 1),
                    '>' => (y, x + 1),
                    'v' => (y + 1, x),
                    '^' => (y - 1, x),
                    _ => unreachable!(),
                };
                if self.map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                    match ch {
                        '<' | '>' => {
                            if next_pos.1 == 0 {
                                next_pos.1 = self.width as isize - 2
                            } else {
                                next_pos.1 = 1
                            }
                        }
                        'v' | '^' => {
                            if next_pos.0 == 0 {
                                next_pos.0 = self.height as isize - 2
                            } else {
                                next_pos.0 = 1
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                (next_pos, ch)
            })
            .collect();
    }
}

fn parse(filename: &str) -> Maze {
    let mut map = read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = (0, map[0].iter().position(|&ch| ch == '.').unwrap() as isize);
    let blizzards = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, &ch)| match ch {
                    '>' | '<' | 'v' | '^' => Some(((y as isize, x as isize), ch)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((isize, isize), char)>>();
    blizzards.iter().for_each(|&(k, _)| {
        map[k.0 as usize][k.1 as usize] = '.';
    });
    let (height, width) = (map.len() as isize, map[0].len() as isize);
    let last_y = height - 1;
    let target = (
        last_y,
        map[last_y as usize]
            .iter()
            .position(|&ch| ch == '.')
            .unwrap() as isize,
    );
    Maze {
        map,
        width,
        height,
        start,
        target,
        blizzards,
        steps: 0,
        applied_blizzards: -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut maze = parse("test-input.txt");
        assert_eq!(maze.find_min_steps(maze.start, maze.target), 18);
    }

    #[test]
    fn part1_test_movement() {
        let mut maze = parse("test-input2.txt");
        maze.move_blizzards();
        assert_eq!(maze.blizzards, [((2, 2), '>'), ((5, 4), 'v')]);
        maze.move_blizzards();
        assert_eq!(maze.blizzards, [((2, 3), '>'), ((1, 4), 'v')]);
        maze.move_blizzards();
        assert_eq!(maze.blizzards, [((2, 4), '>'), ((2, 4), 'v')]);
        maze.move_blizzards();
        assert_eq!(maze.blizzards, [((2, 5), '>'), ((3, 4), 'v')]);
        maze.move_blizzards();
        assert_eq!(maze.blizzards, [((2, 1), '>'), ((4, 4), 'v')]);
    }

    #[test]
    fn part2_test() {
        let mut maze = parse("test-input.txt");
        assert_eq!(maze.find_min_steps(maze.start, maze.target), 18);
        assert_eq!(maze.find_min_steps(maze.target, maze.start), 23);
        assert_eq!(maze.find_min_steps(maze.start, maze.target), 13);
    }
}
