use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let mut world = parse("input.txt");
    world.apply_n_rounds(10);
    println!("part1 solution: {}", world.count_empty_spaces());
    let mut world = parse("input.txt");
    println!("part2 solution: {}", world.apply_until_done());
}

#[derive(Debug, Clone)]
struct World {
    map: HashSet<(isize, isize)>,
    dir_priority: VecDeque<Dir>,
}
impl World {
    fn apply_n_rounds(&mut self, n: usize) {
        for _ in 0..n {
            self.apply_round();
        }
    }

    fn apply_until_done(&mut self) -> usize {
        let mut prev = self.map.clone();
        for i in 1.. {
            self.apply_round();
            if prev == self.map {
                return i;
            }
            prev = self.map.clone();
        }
        unreachable!()
    }

    fn apply_round(&mut self) {
        let potential_moves = self
            .map
            .iter()
            .filter_map(|&(x, y)| {
                if self.is_surrounded((x, y)) {
                    for current_dir in self.dir_priority.iter() {
                        let candidates = current_dir.get_next_moves();
                        if candidates
                            .iter()
                            .all(|&(i, j)| self.map.get(&(x + i, y + j)).is_none())
                        {
                            let diffs = candidates[1];
                            return Some(vec![((x, y), (x + diffs.0, y + diffs.1))]);
                        }
                    }
                }
                None
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, (from, to)| {
                let item = acc.entry(to).or_insert((vec![], 0));
                item.0.push(from);
                item.1 += 1;
                acc
            });
        potential_moves
            .into_iter()
            .filter_map(|(to, (from, count))| {
                if count == 1 {
                    return Some((from[0], to));
                }
                None
            })
            .for_each(|(from, to)| {
                self.map.remove(&from);
                self.map.insert(to);
            });
        self.dir_priority.rotate_left(1);
    }

    fn count_empty_spaces(&self) -> usize {
        let (x_min, x_max, y_min, y_max) = self.map.iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |acc, &(x, y)| (acc.0.min(x), acc.1.max(x), acc.2.min(y), acc.3.max(y)),
        );
        (x_min..=x_max)
            .map(|x| {
                (y_min..=y_max)
                    .filter(|&y| self.map.get(&(x, y)).is_none())
                    .count()
            })
            .sum()
    }

    fn is_surrounded(&self, (x, y): (isize, isize)) -> bool {
        Dir::all_dirs()
            .iter()
            .any(|(i, j)| self.map.get(&(x + i, y + j)).is_some())
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}
impl Dir {
    fn get_next_moves(&self) -> [(isize, isize); 3] {
        match self {
            Dir::North => [(-1, -1), (0, -1), (1, -1)],
            Dir::South => [(-1, 1), (0, 1), (1, 1)],
            Dir::West => [(-1, -1), (-1, 0), (-1, 1)],
            Dir::East => [(1, -1), (1, 0), (1, 1)],
        }
    }

    fn all_dirs() -> Vec<(isize, isize)> {
        let set: HashSet<(isize, isize)> = HashSet::from_iter(
            [Dir::North, Dir::South, Dir::East, Dir::West]
                .into_iter()
                .flat_map(|x| x.get_next_moves()),
        );
        set.into_iter().collect()
    }
}

fn parse(filename: &str) -> World {
    let map = read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .for_each(|(x, _)| {
                    acc.insert((x as isize, y as isize));
                });
            acc
        });
    World {
        map,
        dir_priority: [Dir::North, Dir::South, Dir::West, Dir::East].into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let mut world = parse("test-input.txt");
        world.apply_n_rounds(3);
        assert_eq!(world.count_empty_spaces(), 25);
    }

    #[test]
    fn part1_test2() {
        let mut world = parse("test-input2.txt");
        world.apply_n_rounds(10);
        assert_eq!(world.count_empty_spaces(), 110);
    }

    #[test]
    fn part2_test() {
        let mut world = parse("test-input2.txt");
        assert_eq!(world.apply_until_done(), 20);
    }
}
