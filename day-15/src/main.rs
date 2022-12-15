use std::{collections::HashMap, fs::read_to_string, ops::RangeInclusive};

fn main() {
    let mut world = parse("input.txt");
    println!("part1 solution {}", world.count_unavailable_for_y(2000000));
    println!("part2 solution {:?}", world.beacon_frequency(4_000_000));
}

#[derive(Debug)]
struct World {
    sensors: Vec<Sensor>,
    map: HashMap<Point, char>,
}
impl World {
    fn new() -> Self {
        Self {
            sensors: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn beacon_frequency(&mut self, limit: isize) -> Option<u128> {
        for y in 0..=limit {
            let x = match self.find_ranges(y, Some(0..=limit)).as_slice() {
                [(_, to_x1), (_, _)] => Some(*to_x1 + 1),
                [(x1, x2)] if x2 - x1 < limit => Some(if *x1 != 0 { 0 } else { *x2 + 1 }),
                _ => None,
            };
            if let Some(x) = x {
                return Some(x as u128 * 4_000_000 + y as u128);
            }
        }
        None
    }

    fn count_unavailable_for_y(&mut self, y: isize) -> usize {
        let ranges = self.find_ranges(y, None);
        ranges.iter().map(|(x1, x2)| (x2 - x1).unsigned_abs()).sum()
    }

    fn find_ranges(
        &mut self,
        y: isize,
        x_range: Option<RangeInclusive<isize>>,
    ) -> Vec<(isize, isize)> {
        let range_limit = x_range.unwrap_or(isize::MIN..=isize::MAX);
        self.sensors.iter().fold(Vec::new(), |mut acc, sensor| {
            let dist = sensor.pos.manhattan_distance(&sensor.closest_beacon);
            if (sensor.pos.y - dist..=sensor.pos.y + dist).contains(&y) {
                let x_diff = dist - (sensor.pos.y - y).abs();
                let mut range = (sensor.pos.x - x_diff, sensor.pos.x + x_diff);
                if !range_limit.contains(&range.0) && !range_limit.contains(&range.1) {
                    return acc;
                }
                if range.0 < *range_limit.start() {
                    range.0 = *range_limit.start();
                }
                if range.1 > *range_limit.end() {
                    range.1 = *range_limit.end();
                }
                let items_to_merge = acc
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &r)| {
                        if range.1 < r.0 || range.0 > r.1 {
                            None
                        } else {
                            Some((i, r))
                        }
                    })
                    .collect::<Vec<_>>();
                for &(i, range_to_merge) in items_to_merge.iter().rev() {
                    acc.remove(i);
                    range = (range.0.min(range_to_merge.0), range.1.max(range_to_merge.1));
                }
                if !items_to_merge.is_empty() {
                    acc.insert(items_to_merge[0].0, range);
                } else if let Some(idx) = acc.iter().position(|r| range.1 < r.0) {
                    acc.insert(idx, range);
                } else {
                    acc.push(range);
                }
            }
            acc
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

fn parse(filename: &str) -> World {
    let re = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .fold(World::new(), |mut world, line| {
            let captures = re.captures(line).unwrap();
            let values = captures
                .iter()
                .filter_map(|cap| cap?.as_str().parse::<isize>().ok())
                .collect::<Vec<_>>();
            let beacon = Point::new(values[2], values[3]);
            let sensor = Sensor {
                pos: Point::new(values[0], values[1]),
                closest_beacon: beacon,
            };
            world.sensors.push(sensor);
            world.map.insert(sensor.pos, 'S');
            world.map.insert(beacon, 'B');

            world
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut world = parse("test-input.txt");
        assert_eq!(world.count_unavailable_for_y(10), 26);
    }

    #[test]
    fn part2_test() {
        let mut world = parse("test-input.txt");
        assert_eq!(world.beacon_frequency(20), Some(56000011));
    }
}
