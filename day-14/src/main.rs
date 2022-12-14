use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut cave = parse("input.txt");
    cave.simulate_til_end();
    println!("part1 solution {}", cave.count_sand());

    cave.set_y_limit();
    cave.simulate_til_end();
    println!("part2 solution {}", cave.count_sand());
}

const SAND_SOURCE: (isize, isize) = (500, 0);

#[derive(Debug)]
struct Cave {
    map: HashMap<(isize, isize), char>,
    min: (isize, isize),
    max: (isize, isize),
    has_floor: bool,
}
impl Cave {
    fn simulate_til_end(&mut self) {
        while self.simulate_drop().is_some() {}
    }

    fn simulate_drop(&mut self) -> Option<(isize, isize)> {
        let mut curr_pos = (SAND_SOURCE.0, SAND_SOURCE.1);
        while curr_pos.1 < self.max.1 {
            let next_y = curr_pos.1 + 1;
            let is_floor_level = self.has_floor && next_y == self.max.1;
            let is_used_bellow = self.map.get(&(curr_pos.0, next_y)).is_some() || is_floor_level;
            let is_used_bellow_left =
                self.map.get(&(curr_pos.0 - 1, next_y)).is_some() || is_floor_level;
            let is_used_bellow_right =
                self.map.get(&(curr_pos.0 + 1, next_y)).is_some() || is_floor_level;
            match (is_used_bellow, is_used_bellow_left, is_used_bellow_right) {
                (true, true, true) if self.map.contains_key(&curr_pos) => {
                    return None;
                }
                (true, true, true) if !self.map.contains_key(&curr_pos) => {
                    self.add_item(curr_pos, 'o');
                    return Some(curr_pos);
                }
                (true, false, _) => curr_pos.0 -= 1,
                (true, true, false) => curr_pos.0 += 1,
                _ => curr_pos.1 = next_y,
            }
        }
        None
    }

    fn add_item(&mut self, (x, y): (isize, isize), ch: char) {
        self.map.insert((x, y), ch);
        self.min = (x.min(self.min.0), y.min(self.min.1));
        self.max = (x.max(self.max.0), y.max(self.max.1));
    }

    fn count_sand(&self) -> usize {
        self.map.iter().filter(|(_, &ch)| ch == 'o').count()
    }

    fn set_y_limit(&mut self) {
        self.max.1 += 2;
        self.has_floor = true;
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        let (w, h) = (
            (self.max.0 - self.min.0 + 1) as usize,
            (self.max.1 + 1) as usize,
        );
        let mut map = vec![vec![' '; w]; h];
        for (&(x, y), &ch) in self.map.iter() {
            map[(y) as usize][(x - self.min.0) as usize] = ch;
        }
        let lines = map
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>();
        println!("{}", lines.join("\n"));
    }
}

fn parse(filename: &str) -> Cave {
    let mut cave = Cave {
        map: HashMap::new(),
        min: (isize::MAX, isize::MAX),
        max: (isize::MIN, isize::MIN),
        has_floor: false,
    };
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .for_each(|l| {
            let line_points = l
                .split_terminator(" -> ")
                .filter_map(|s| {
                    let (x, y) = s.split_once(',')?;
                    Some((x.parse::<isize>().ok()?, y.parse::<isize>().ok()?))
                })
                .collect::<Vec<_>>();
            for points in line_points.windows(2) {
                let (from, to) = (points[0], points[1]);
                for x in from.0.min(to.0)..=from.0.max(to.0) {
                    for y in from.1.min(to.1)..=from.1.max(to.1) {
                        cave.add_item((x, y), '#');
                    }
                }
            }
        });
    cave
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut cave = parse("test-input.txt");
        cave.simulate_til_end();
        assert_eq!(cave.count_sand(), 24);
    }

    #[test]
    fn part2_test() {
        let mut cave = parse("test-input.txt");
        cave.set_y_limit();
        cave.simulate_til_end();
        assert_eq!(cave.count_sand(), 93);
    }
}
