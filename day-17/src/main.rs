use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let jet_pattern = parse("input.txt");
    let mut tetris = Tetris::new(&jet_pattern);
    tetris.place_n_shapes(2022);
    println!("part1 solution {}", tetris.highest_y);
    let mut tetris = Tetris::new(&jet_pattern);
    tetris.place_n_shapes(1_000_000_000_000);
    println!("part2 solution {}", tetris.highest_y);
}

type CacheKey = (BTreeSet<(usize, usize)>, usize, usize);
type CacheValue = (usize, usize);

struct Tetris {
    map: VecDeque<Vec<char>>,
    shapes: Vec<Shape>,
    jet_pattern: Vec<char>,
    highest_y: usize,
    current_y: usize,
    removed_y: usize,
    curr_jet_move: usize,
    curr_shape: usize,
    cache: HashMap<CacheKey, CacheValue>,
}
impl Tetris {
    fn new(jet_pattern: &str) -> Self {
        Self {
            map: VecDeque::new(),
            shapes: parse_shapes(),
            highest_y: 0,
            current_y: 0,
            removed_y: 0,
            curr_jet_move: 0,
            curr_shape: 0,
            jet_pattern: jet_pattern.chars().collect(),
            cache: HashMap::new(),
        }
    }

    fn place_shape(&mut self) -> BTreeSet<(usize, usize)> {
        let mut x = 2;
        let mut y = self.current_y + 3;
        let empty_row = vec!['.'; 7];
        let shape = &self.shapes[self.curr_shape];
        let shape_height = shape.map.len();

        for i in self.current_y..y + shape_height {
            if self.map.get(i).is_none() {
                self.map.push_back(empty_row.clone());
            }
        }
        loop {
            let curr_move = self.jet_pattern[self.curr_jet_move];
            self.curr_jet_move = (self.curr_jet_move + 1) % self.jet_pattern.len();
            (y, x) = match curr_move {
                '<' => self.move_shape_left((y, x)),
                '>' => self.move_shape_right((y, x)),
                _ => unreachable!(),
            };

            let next_coords = self.move_shape_down((y, x));
            if next_coords.0 == y {
                break;
            }
            y = next_coords.0;
        }
        self.draw_shape((y, x));

        let visited = self.cleanup(empty_row);
        self.curr_shape = (self.curr_shape + 1) % self.shapes.len();
        visited
    }

    fn move_shape_right(&self, (y, x): (usize, usize)) -> (usize, usize) {
        if x < self.map[0].len() - self.shapes[self.curr_shape].map[0].len()
            && !self.shapes[self.curr_shape]
                .right_border
                .iter()
                .any(|&(i, j)| self.map[y + i][x + j + 1] == '#')
        {
            (y, x + 1)
        } else {
            (y, x)
        }
    }

    fn move_shape_left(&self, (y, x): (usize, usize)) -> (usize, usize) {
        if x > 0
            && !self.shapes[self.curr_shape]
                .left_border
                .iter()
                .any(|&(i, j)| self.map[y + i][x + j - 1] == '#')
        {
            (y, x - 1)
        } else {
            (y, x)
        }
    }

    fn move_shape_down(&self, (y, x): (usize, usize)) -> (usize, usize) {
        if y == 0
            || self.shapes[self.curr_shape]
                .bottom_border
                .iter()
                .any(|&(i, j)| self.map[y + i - 1][x + j] == '#')
        {
            (y, x)
        } else {
            (y - 1, x)
        }
    }

    fn place_n_shapes(&mut self, n: usize) {
        let mut i = 0;
        let mut visited = BTreeSet::new();
        while i < n {
            let prev_visited = visited.clone();
            let curr_jet_move = self.curr_jet_move;
            let curr_shape = self.curr_shape;
            if let Some(&(steps, height)) =
                self.cache
                    .get(&(prev_visited.clone(), curr_jet_move, curr_shape))
            {
                let cycle = i - steps;
                let multiplier = (n - i) / cycle;
                let height_diff = self.highest_y - height;
                i += multiplier * cycle;
                self.highest_y += multiplier * height_diff;
                self.removed_y += multiplier * height_diff;
                while i < n {
                    self.place_shape();
                    i += 1;
                }
                continue;
            }
            visited = self.place_shape();

            self.cache.insert(
                (prev_visited.clone(), curr_jet_move, curr_shape),
                (i, self.highest_y),
            );
            i += 1;
        }
    }

    fn draw_shape(&mut self, (y, x): (usize, usize)) {
        for curr_y in y..y + self.shapes[self.curr_shape].map.len() {
            for curr_x in x..x + self.shapes[self.curr_shape].map[0].len() {
                if self.shapes[self.curr_shape].map[curr_y - y][curr_x - x] == '#' {
                    self.map[curr_y][curr_x] = '#';
                }
            }
        }
        self.highest_y = self
            .highest_y
            .max(y + self.removed_y + self.shapes[self.curr_shape].map.len());
        self.current_y = self
            .current_y
            .max(y + self.shapes[self.curr_shape].map.len());
    }

    fn cleanup(&mut self, empty_row: Vec<char>) -> BTreeSet<(usize, usize)> {
        loop {
            if let Some(row) = self.map.back() {
                if row != &empty_row {
                    self.map.push_back(empty_row);
                    break;
                }
                self.map.pop_back();
            }
        }
        let (lowest_reachable_y, visited) = find_lowest_reachable_y(&self.map);
        for _ in 0..lowest_reachable_y {
            self.map.pop_front();
        }
        self.removed_y += lowest_reachable_y;
        self.current_y -= lowest_reachable_y;
        visited
    }

    #[allow(dead_code)]
    fn print_tetris(
        map: &VecDeque<Vec<char>>,
        shape: &Shape,
        y: usize,
        x: usize,
        print_as_current: bool,
    ) {
        let mut map = map.clone();
        if print_as_current {
            shape.map.iter().enumerate().for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, ch)| {
                    if ch == &'#' {
                        map[y + i][x + j] = '@';
                    }
                });
            });
        }
        let mut s = String::new();
        for row in map.iter().rev() {
            for ch in row.iter() {
                s.push(*ch);
            }
            s.push('\n');
        }
        println!("{s}\n\n");
    }
}

fn find_lowest_reachable_y(map: &VecDeque<Vec<char>>) -> (usize, BTreeSet<(usize, usize)>) {
    let mut min_y = map.len() - 1;
    let mut visited = BTreeSet::new();
    let mut q = vec![(map.len() - 1, 0)];
    while let Some((y, x)) = q.pop() {
        visited.insert((y, x));
        min_y = min_y.min(y);
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .for_each(|(y_diff, x_diff)| {
                let (next_y, next_x) = (y as isize + y_diff, x as isize + x_diff);
                if next_y <= (map.len() - 1) as isize
                    && next_y >= 0
                    && next_x <= (map[0].len() - 1) as isize
                    && next_x >= 0
                    && map[next_y as usize][next_x as usize] == '.'
                    && !visited.contains(&(next_y as usize, next_x as usize))
                {
                    q.push((next_y as usize, next_x as usize));
                }
            });
    }
    (min_y, visited)
}

#[derive(Debug, Clone)]
struct Shape {
    map: Vec<Vec<char>>,
    bottom_border: Vec<(usize, usize)>,
    left_border: Vec<(usize, usize)>,
    right_border: Vec<(usize, usize)>,
}
impl Shape {
    fn new(map: Vec<Vec<char>>) -> Self {
        let mut bottom_border = vec![];
        for j in 0..map[0].len() {
            for (i, row) in map.iter().enumerate() {
                if row[j] == '#' {
                    bottom_border.push((i, j));
                    break;
                }
            }
        }
        let mut left_border = vec![];
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == '#' {
                    left_border.push((i, j));
                    break;
                }
            }
        }
        let mut right_border = vec![];
        for i in 0..map.len() {
            for j in (0..map[0].len()).rev() {
                if map[i][j] == '#' {
                    right_border.push((i, j));
                    break;
                }
            }
        }
        Self {
            map,
            bottom_border,
            left_border,
            right_border,
        }
    }
}

fn parse(filename: &str) -> String {
    read_to_string(filename)
        .expect("failed to read file")
        .trim()
        .into()
}

fn parse_shapes() -> Vec<Shape> {
    read_to_string("shapes.txt")
        .expect("failed to read file")
        .split_terminator("\n\n")
        .map(|s| Shape::new(s.lines().rev().map(|l| l.chars().collect()).collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let jet_pattern = parse("test-input.txt");
        let mut tetris = Tetris::new(&jet_pattern);
        tetris.place_n_shapes(2022);
        assert_eq!(tetris.highest_y, 3068);
    }

    #[test]
    fn part2_test() {
        let jet_pattern = parse("test-input.txt");
        let mut tetris = Tetris::new(&jet_pattern);
        tetris.place_n_shapes(1_000_000_000_000);
        assert_eq!(tetris.highest_y, 1514285714288);
    }
}
