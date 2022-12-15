use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
};

fn main() {
    let mut world = parse("input.txt");
    println!("part1 solution {}", world.count_unavailable_for_y(2000000));
    // 4_000_000
}

#[derive(Debug)]
struct World {
    sensors: Vec<Sensor>,
    map: HashMap<Point, char>,
    visited: HashSet<Point>,
}
impl World {
    fn new() -> Self {
        Self {
            sensors: Vec::new(),
            map: HashMap::new(),
            visited: HashSet::new(),
        }
    }
    fn reset(&mut self) {
        self.visited.clear();
    }

    fn find_distress_beacon_frequency(&mut self, limit: isize) -> u128 {
        for y in 0..=limit {
            let count = self.count_unavailable_for_y_in_x_range(y, Some(0..=limit));
            println!("visited {count} for y = {y}");
            if count <= limit as usize {
                let x = (0..=limit)
                    .position(|x| !self.visited.contains(&Point::new(x, y)))
                    .unwrap();
                println!("found x {x} y {y}");
                return x as u128 * limit as u128 + y as u128;
            }
        }
        unreachable!()
    }

    fn count_unavailable_for_y(&mut self, y: isize) -> usize {
        self.count_unavailable_for_y_in_x_range(y, None)
    }

    fn count_unavailable_for_y_in_x_range(
        &mut self,
        y: isize,
        x_range: Option<RangeInclusive<isize>>,
    ) -> usize {
        self.sensors
            .iter()
            .filter_map(|sensor| {
                let dist = sensor.pos.manhattan_distance(&sensor.closest_beacon);
                if (sensor.pos.y - dist..=sensor.pos.y + dist).contains(&y) {
                    let mut count = 0;
                    for x_diff in 0..=dist {
                        let next_point = Point::new(sensor.pos.x + x_diff, y);
                        if (x_range.is_none() || x_range.as_ref().unwrap().contains(&next_point.x))
                            && sensor.pos.manhattan_distance(&next_point) <= dist
                            && !self.visited.contains(&next_point)
                        {
                            self.visited.insert(next_point);
                            if self.map.get(&next_point).unwrap_or(&' ') != &'B' {
                                count += 1;
                            }
                        }
                        let next_point = Point::new(sensor.pos.x - x_diff, y);
                        if (x_range.is_none() || x_range.as_ref().unwrap().contains(&next_point.x))
                            && sensor.pos.manhattan_distance(&next_point) <= dist
                            && !self.visited.contains(&next_point)
                        {
                            self.visited.insert(next_point);
                            if self.map.get(&next_point).unwrap_or(&' ') != &'B' {
                                count += 1;
                            }
                        }
                    }
                    Some(count)
                } else {
                    None
                }
            })
            .sum()
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
        assert_eq!(world.find_distress_beacon_frequency(20), 56000011);
    }
}
