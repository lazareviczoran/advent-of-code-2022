use std::fs::read_to_string;

fn main() {
    let jet_pattern = parse("input.txt");
    let mut tetris = Tetris::new(&jet_pattern);
    tetris.place_n_shapes(2022);
    println!("part1 solution {}", tetris.highest_y);
    let mut tetris = Tetris::new(&jet_pattern);
    tetris.place_n_shapes(1_000_000_000_000);
    println!("part2 solution {}", tetris.highest_y);
}

struct Tetris {
    map: Vec<Vec<char>>,
    shapes: Vec<Vec<Vec<char>>>,
    jet_pattern: Vec<char>,
    highest_y: usize,
    curr_jet_move: usize,
}
impl Tetris {
    fn new(jet_pattern: &str) -> Self {
        Self {
            map: vec![],
            shapes: parse_shapes(),
            highest_y: 0,
            curr_jet_move: 0,
            jet_pattern: jet_pattern.chars().collect(),
        }
    }

    fn place_shape(&mut self, shape: &[Vec<char>]) {
        let mut x = 2;
        let mut y = self.highest_y + 3;
        let mut bottom_items = vec![];
        for j in 0..shape[0].len() {
            for (i, row) in shape.iter().enumerate() {
                if row[j] == '#' {
                    bottom_items.push((i, j));
                    break;
                }
            }
        }
        let mut left_items = vec![];
        for i in 0..shape.len() {
            for j in 0..shape[0].len() {
                if shape[i][j] == '#' {
                    left_items.push((i, j));
                    break;
                }
            }
        }
        let mut right_items = vec![];
        for i in 0..shape.len() {
            for j in (0..shape[0].len()).rev() {
                if shape[i][j] == '#' {
                    right_items.push((i, j));
                    break;
                }
            }
        }
        for i in self.highest_y..y + shape.len() {
            if self.map.get(i).is_none() {
                self.map.push(vec!['.'; 7]);
            }
        }
        loop {
            let curr_move = self.jet_pattern[self.curr_jet_move];
            self.curr_jet_move = (self.curr_jet_move + 1) % self.jet_pattern.len();
            match curr_move {
                '<' => {
                    if x > 0
                        && !left_items
                            .iter()
                            .any(|&(i, j)| self.map[y + i][x + j - 1] == '#')
                    {
                        x -= 1;
                    }
                }
                '>' => {
                    if x < self.map[0].len() - shape[0].len()
                        && !right_items
                            .iter()
                            .any(|&(i, j)| self.map[y + i][x + j + 1] == '#')
                    {
                        x += 1;
                    }
                }
                _ => unreachable!(),
            }

            if y == 0
                || bottom_items
                    .iter()
                    .any(|&(i, j)| self.map[y + i - 1][x + j] == '#')
            {
                break;
            }

            y -= 1;
        }
        // place the shape
        for curr_y in y..y + shape.len() {
            for curr_x in x..x + shape[0].len() {
                if shape[curr_y - y][curr_x - x] == '#' {
                    self.map[curr_y][curr_x] = '#';
                }
            }
        }

        self.highest_y = self.highest_y.max(y + shape.len());
        // Self::print_tetris(&self.map, shape, y, x, false);
    }

    #[allow(dead_code)]
    fn print_tetris(
        map: &[Vec<char>],
        shape: &[Vec<char>],
        y: usize,
        x: usize,
        print_as_current: bool,
    ) {
        let mut map = map.to_vec();
        if print_as_current {
            for (i, row) in shape.iter().enumerate() {
                for (j, ch) in row.iter().enumerate() {
                    if ch == &'#' {
                        map[y + i][x + j] = '@';
                    }
                }
            }
        }
        let mut s = String::new();
        for row in map.iter().rev() {
            for ch in row.iter() {
                s.push(*ch);
            }
            s.push('\n');
        }
        println!("{s}");
    }

    fn place_n_shapes(&mut self, n: usize) {
        let shapes = self.shapes.clone();
        shapes
            .iter()
            .cycle()
            .take(n)
            .for_each(|s| self.place_shape(s));
    }
}

fn parse(filename: &str) -> String {
    read_to_string(filename)
        .expect("failed to read file")
        .trim()
        .into()
}

fn parse_shapes() -> Vec<Vec<Vec<char>>> {
    read_to_string("shapes.txt")
        .expect("failed to read file")
        .split_terminator("\n\n")
        .map(|s| s.lines().rev().map(|l| l.chars().collect()).collect())
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
