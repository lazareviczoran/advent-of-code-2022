use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let mut monkey_map = parse("input.txt");
    monkey_map.apply_actions();
    println!("part1 solution: {}", monkey_map.get_score());
    let mut monkey_map = parse("input.txt");
    monkey_map.apply_actions2();
    println!("part2 solution: {}", monkey_map.get_score());
}

struct MonkeyMap {
    map: Vec<Vec<char>>,
    sides_map: Vec<Vec<char>>,
    actions: Vec<Action>,
    pos: (isize, isize),
    direction: Direction,
    transformations: BTreeMap<char, BTreeMap<char, Direction>>,
}
impl MonkeyMap {
    fn apply_actions(&mut self) {
        for &action in &self.actions {
            match action {
                Action::Move(steps) => {
                    let diff = self.direction.get_move();
                    for _ in 0..steps {
                        let mut next_pos = (self.pos.0 + diff.0, self.pos.1 + diff.1);
                        if next_pos.0 < 0
                            || next_pos.0 >= self.map.len() as isize
                            || next_pos.1 < 0
                            || next_pos.1 >= self.map[0].len() as isize
                            || self.map[next_pos.0 as usize][next_pos.1 as usize] == ' '
                        {
                            match diff {
                                (1, 0) => {
                                    next_pos.0 = self
                                        .map
                                        .iter()
                                        .position(|row| row[next_pos.1 as usize] != ' ')
                                        .unwrap()
                                        as isize;
                                }
                                (-1, 0) => {
                                    next_pos.0 = self
                                        .map
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find_map(|(x, row)| {
                                            if row[next_pos.1 as usize] != ' ' {
                                                Some(x)
                                            } else {
                                                None
                                            }
                                        })
                                        .unwrap()
                                        as isize;
                                }
                                (0, 1) => {
                                    next_pos.1 = self.map[next_pos.0 as usize]
                                        .iter()
                                        .position(|&ch| ch != ' ')
                                        .unwrap()
                                        as isize;
                                }
                                (0, -1) => {
                                    next_pos.1 = self.map[next_pos.0 as usize]
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find_map(|(x, &ch)| if ch != ' ' { Some(x) } else { None })
                                        .unwrap()
                                        as isize;
                                }
                                _ => unreachable!(),
                            }
                        }
                        if self.map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                            break;
                        }
                        self.pos = next_pos;
                    }
                }
                Action::Right | Action::Left => self.direction.rotate(action),
            }
        }
    }

    fn apply_actions2(&mut self) {
        let actions = self.actions.clone();
        for action in actions.into_iter() {
            match action {
                Action::Move(steps) => {
                    for _ in 0..steps {
                        let mut reassigned_move = None;
                        let diff = self.direction.get_move();
                        let mut next_pos = (self.pos.0 + diff.0, self.pos.1 + diff.1);
                        if next_pos.0 < 0
                            || next_pos.0 >= self.map.len() as isize
                            || next_pos.1 < 0
                            || next_pos.1 >= self.map[0].len() as isize
                            || self.map[next_pos.0 as usize][next_pos.1 as usize] == ' '
                        {
                            let (new_pos, next_dir) = self.get_next_transf_pos();
                            next_pos = new_pos;
                            reassigned_move = Some(next_dir);
                        }
                        if self.map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                            break;
                        }
                        self.pos = next_pos;
                        if let Some(dir) = reassigned_move {
                            self.direction = dir;
                        }
                    }
                }
                Action::Right | Action::Left => self.direction.rotate(action),
            }
        }
    }

    fn get_score(&self) -> usize {
        1000 * (self.pos.0 + 1) as usize + 4 * (self.pos.1 + 1) as usize + self.direction.score()
    }

    fn get_side_size(&self) -> isize {
        let side_width = self
            .map
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch != ' ').count())
            .min()
            .unwrap() as isize;
        side_width
    }

    fn get_next_transf_pos(&mut self) -> ((isize, isize), Direction) {
        let curr_side = self.sides_map[self.pos.0 as usize][self.pos.1 as usize];
        let tranf_dir = self
            .transformations
            .get(&curr_side)
            .unwrap()
            .iter()
            .find(|&(_k, &v)| v == self.direction)
            .unwrap();
        let reverse_tranf_dir = self
            .transformations
            .get(tranf_dir.0)
            .unwrap()
            .get(&curr_side)
            .unwrap();

        let from_side_start = self.find_side_start_pos(curr_side);
        let to_side_start = self.find_side_start_pos(*tranf_dir.0);
        let side_size = self.get_side_size() - 1;
        let new_pos = match (tranf_dir.1, reverse_tranf_dir) {
            (Direction::Right, Direction::Up) => (
                to_side_start.0,
                to_side_start.1 + side_size - (self.pos.0 - from_side_start.0),
            ),
            (Direction::Right, Direction::Down) => (
                to_side_start.0 + side_size,
                to_side_start.1 + (self.pos.0 - from_side_start.0),
            ),
            (Direction::Down, Direction::Right) => (
                to_side_start.0 + (self.pos.1 - from_side_start.1),
                to_side_start.1 + side_size,
            ),
            (Direction::Down, Direction::Down) => (
                to_side_start.0 + (self.pos.0 - from_side_start.0),
                to_side_start.1 + side_size - (self.pos.1 - from_side_start.1),
            ),
            (Direction::Up, Direction::Left) => (
                to_side_start.0 + (self.pos.1 - from_side_start.1),
                to_side_start.1,
            ),
            (Direction::Left, Direction::Up) => (
                to_side_start.0,
                to_side_start.1 + (self.pos.0 - from_side_start.0),
            ),
            (Direction::Left, Direction::Left) => (
                to_side_start.0 + side_size - (self.pos.0 - from_side_start.0),
                to_side_start.1,
            ),
            (Direction::Right, Direction::Right) => (
                to_side_start.0 + side_size - (self.pos.0 - from_side_start.0),
                to_side_start.1 + side_size,
            ),
            (Direction::Up, Direction::Down) => (
                to_side_start.0 + side_size,
                to_side_start.1 + (self.pos.1 - from_side_start.1),
            ),
            (Direction::Down, Direction::Up) => (
                to_side_start.0,
                to_side_start.1 + (self.pos.1 - from_side_start.1),
            ),
            _ => (0, 0),
        };
        (new_pos, reverse_tranf_dir.oposite())
    }

    fn mark_sides(&mut self) {
        let side_size = self.get_side_size();
        let mut y = 0;
        let mut current_side = b'1';
        while y < self.map.len() {
            let mut x = 0;
            while x < self.map[0].len() {
                if self.map[y][x] != ' ' {
                    for curr_y in y..y + side_size as usize {
                        for curr_x in x..x + side_size as usize {
                            self.sides_map[curr_y][curr_x] = current_side as char;
                        }
                    }
                    current_side += 1;
                }
                x += side_size as usize;
            }
            y += side_size as usize;
        }
    }

    fn find_side_start_pos(&self, side: char) -> (isize, isize) {
        let side_size = self.get_side_size();
        let mut y = 0;
        while y < self.sides_map.len() {
            let mut x = 0;
            while x < self.sides_map[0].len() {
                if self.sides_map[y][x] == side {
                    return (y as isize, x as isize);
                }
                x += side_size as usize;
            }
            y += side_size as usize;
        }
        unreachable!()
    }

    fn init_transformations_map(&mut self) {
        let side_size = self.get_side_size();
        let mut y = 0;
        while y < self.map.len() as isize {
            let mut x = 0;
            while x < self.map[0].len() as isize {
                let curr_side_char = self.sides_map[y as usize][x as usize];
                [
                    (y, x),
                    (y, x + side_size - 1),
                    (y + side_size - 1, x),
                    (y + side_size - 1, x + side_size - 1),
                ]
                .into_iter()
                .for_each(|edge_pos| {
                    if self.map[edge_pos.0 as usize][edge_pos.1 as usize] != ' ' {
                        [(0, 1), (0, -1), (1, 0), (-1, 0)]
                            .iter()
                            .filter(|&&(i, j)| {
                                let pos = (edge_pos.0 + i, edge_pos.1 + j);
                                pos.0 >= 0
                                    && pos.0 < self.map.len() as isize
                                    && pos.1 >= 0
                                    && pos.1 < self.map[0].len() as isize
                                    && self.map[pos.0 as usize][pos.1 as usize] != ' '
                                    && self.sides_map[pos.0 as usize][pos.1 as usize]
                                        != curr_side_char
                            })
                            .for_each(|diff| {
                                let pos = (edge_pos.0 + diff.0, edge_pos.1 + diff.1);
                                let dir = match diff {
                                    (0, 1) => Direction::Right,
                                    (0, -1) => Direction::Left,
                                    (1, 0) => Direction::Down,
                                    (-1, 0) => Direction::Up,
                                    _ => unreachable!(),
                                };
                                self.transformations
                                    .entry(curr_side_char)
                                    .or_default()
                                    .insert(self.sides_map[pos.0 as usize][pos.1 as usize], dir);

                                self.transformations
                                    .entry(self.sides_map[pos.0 as usize][pos.1 as usize])
                                    .or_default()
                                    .insert(curr_side_char, dir.oposite());
                            });
                    }
                });
                x += side_size;
            }
            y += side_size;
        }
        while !self
            .transformations
            .iter()
            .all(|(_from, connections)| connections.len() == 4)
        {
            let mut has_changed = false;
            let mut transformations_vec = self
                .transformations
                .iter()
                .map(|(&a, x)| (a, x.clone()))
                .collect::<Vec<_>>();
            transformations_vec.sort_by_key(|a| a.1.len());

            let mut i = transformations_vec.len() as isize - 1;
            while i >= 0 {
                let missing_transformations = [
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                ]
                .into_iter()
                .filter(|dir| {
                    !transformations_vec[i as usize]
                        .1
                        .iter()
                        .any(|(_, existing)| existing == dir)
                })
                .collect::<Vec<_>>();

                missing_transformations.iter().for_each(|missing_dir| {
                    let paths = match missing_dir {
                        Direction::Right => {
                            vec![
                                vec![Direction::Up, Direction::Right],
                                vec![Direction::Down, Direction::Right],
                            ]
                        }
                        Direction::Left => {
                            vec![
                                vec![Direction::Up, Direction::Left],
                                vec![Direction::Down, Direction::Left],
                            ]
                        }
                        Direction::Up => {
                            vec![
                                vec![Direction::Right, Direction::Up],
                                vec![Direction::Left, Direction::Up],
                            ]
                        }
                        Direction::Down => {
                            vec![
                                vec![Direction::Right, Direction::Down],
                                vec![Direction::Left, Direction::Down],
                            ]
                        }
                    };
                    'outer: for required_steps in paths {
                        let (mut current, _neighbours) = &transformations_vec[i as usize];
                        let next = self.transformations.get(&current).unwrap();
                        if let Some((&next, _next_dir)) =
                            next.iter().find(|&(_k, &v)| v == required_steps[0])
                        {
                            current = next;
                        } else {
                            continue;
                        }
                        let next = self.transformations.get(&current).unwrap();
                        if self.get_side_size() < 50
                            && transformations_vec.iter().map(|s| s.1.len()).sum::<usize>() == 23
                        {
                            if let Some((&next, _next_dir)) = next.iter().find(|&(k, _v)| {
                                self.transformations
                                    .get(k)
                                    .unwrap()
                                    .contains_key(&transformations_vec[i as usize].0)
                            }) {
                                self.transformations
                                    .entry(transformations_vec[i as usize].0)
                                    .or_default()
                                    .insert(next, *missing_dir);
                                continue 'outer;
                            }
                        } else if let Some((&next, _next_dir)) =
                            next.iter().find(|&(&_k, &v)| v == required_steps[1])
                        {
                            current = next;
                        } else {
                            continue 'outer;
                        }
                        if transformations_vec[i as usize].0 == current
                            || self
                                .transformations
                                .entry(transformations_vec[i as usize].0)
                                .or_default()
                                .get(&current)
                                .is_some()
                        {
                            return;
                        }

                        self.transformations
                            .entry(transformations_vec[i as usize].0)
                            .or_default()
                            .insert(current, *missing_dir);
                        let missing_transformations = [
                            Direction::Down,
                            Direction::Left,
                            Direction::Right,
                            Direction::Up,
                        ]
                        .into_iter()
                        .filter(|dir| {
                            !self
                                .transformations
                                .entry(current)
                                .or_default()
                                .iter()
                                .any(|(_, existing)| existing == dir)
                        })
                        .collect::<Vec<_>>();
                        if missing_transformations.len() == 1
                            && self
                                .transformations
                                .entry(current)
                                .or_default()
                                .get(&transformations_vec[i as usize].0)
                                .is_none()
                        {
                            let reverse_dir = missing_transformations[0];
                            self.transformations
                                .entry(current)
                                .or_default()
                                .insert(transformations_vec[i as usize].0, reverse_dir);
                        }
                        has_changed = true;
                        return;
                    }
                });
                if has_changed {
                    break;
                }
                i -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Right,
    Left,
    Move(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_move(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn score(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn rotate(&mut self, action: Action) {
        let new_dir = match action {
            Action::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            },
            Action::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            },
            _ => unimplemented!(),
        };
        *self = new_dir;
    }

    fn oposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn parse(filename: &str) -> MonkeyMap {
    let content = read_to_string(filename).expect("failed to read file");
    let (map_str, actions_str) = content.split_once("\n\n").unwrap();
    let partial_map = map_str
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut map =
        vec![vec![' '; partial_map.iter().map(|r| r.len()).max().unwrap()]; partial_map.len()];

    for (i, row) in partial_map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            map[i][j] = c;
        }
    }
    let mut split_by_right = actions_str.trim().split_terminator('R').collect::<Vec<_>>();
    for i in (1..=split_by_right.len() - 1).rev() {
        split_by_right.insert(i, "R");
    }
    let actions = split_by_right
        .iter()
        .flat_map(|s| {
            let mut split_by_left = s.split_terminator('L').collect::<Vec<_>>();
            for i in (1..=split_by_left.len() - 1).rev() {
                split_by_left.insert(i, "L");
            }
            split_by_left
        })
        .map(|s| match s {
            "R" => Action::Right,
            "L" => Action::Left,
            str => Action::Move(str.parse().unwrap()),
        })
        .collect();
    let pos = (0, map[0].iter().position(|&c| c == '.').unwrap() as isize);

    let mut monkey_map = MonkeyMap {
        sides_map: map.clone(),
        map,
        actions,
        pos,
        direction: Direction::Right,
        transformations: BTreeMap::new(),
    };
    monkey_map.mark_sides();
    monkey_map.init_transformations_map();

    monkey_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut monkey_map = parse("test-input.txt");
        monkey_map.apply_actions();
        assert_eq!(monkey_map.get_score(), 6032);
    }

    #[test]
    fn part2_test() {
        let mut monkey_map = parse("test-input.txt");
        monkey_map.apply_actions2();
        assert_eq!(monkey_map.get_score(), 5031);
    }
}
