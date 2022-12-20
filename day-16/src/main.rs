use itertools::*;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    rc::Rc,
};

fn main() {
    let valves = parse("input.txt");
    let instant = std::time::Instant::now();
    println!("part1 solution {}", calc_max_pressure_release(&valves));
    println!("elapsed {} ms", instant.elapsed().as_millis());
    let instant = std::time::Instant::now();
    println!("part2 solution {}", calc_max_pressure_release2(&valves));
    println!("elapsed {} ms", instant.elapsed().as_millis());
}

fn calc_max_pressure_release(valves: &HashMap<Rc<String>, (usize, Vec<Rc<String>>)>) -> usize {
    let (start, _) = valves.iter().find(|(k, _)| k.starts_with("AA")).unwrap();
    calc_max_pressure_release_rec(
        valves,
        30,
        start.clone(),
        BTreeSet::new(),
        &mut HashMap::new(),
    )
}

type Cache = HashMap<(Rc<String>, BTreeSet<Rc<String>>, usize), usize>;

fn calc_max_pressure_release_rec(
    valves: &HashMap<Rc<String>, (usize, Vec<Rc<String>>)>,
    remaining_time: usize,
    curr_valve: Rc<String>,
    used: BTreeSet<Rc<String>>,
    cache: &mut Cache,
) -> usize {
    if let Some(existing) = cache.get_mut(&(curr_valve.clone(), used.clone(), remaining_time)) {
        return *existing;
    }
    let new_value = used.iter().map(|x| valves.get(x).unwrap().0).sum::<usize>();
    if remaining_time == 0 {
        return 0;
    }
    let mut max_sum = 0;
    if let Some((value, neighbours)) = valves.get(&curr_valve) {
        if *value > 0 && !used.contains(&curr_valve) {
            max_sum = max_sum.max(calc_max_pressure_release_rec(
                valves,
                remaining_time - 1,
                curr_valve.clone(),
                &used | &BTreeSet::from_iter(vec![curr_valve.clone()].into_iter()),
                cache,
            ));
        }
        max_sum = max_sum.max(
            neighbours
                .iter()
                .map(|neighbour| {
                    calc_max_pressure_release_rec(
                        valves,
                        remaining_time - 1,
                        neighbour.clone(),
                        used.clone(),
                        cache,
                    )
                })
                .max()
                .unwrap_or(0),
        );
    }
    let new_sum = new_value + max_sum;
    cache.insert((curr_valve, used, remaining_time), new_sum);
    new_sum
}

fn calc_max_pressure_release2(valves: &HashMap<Rc<String>, (usize, Vec<Rc<String>>)>) -> usize {
    let (start, _) = valves.iter().find(|(k, _)| k.starts_with("AA")).unwrap();
    calc_max_pressure_release_rec2(
        valves,
        26,
        start.clone(),
        start.clone(),
        BTreeSet::new(),
        &mut HashMap::new(),
    )
}

type PairCache = HashMap<(Rc<String>, Rc<String>, BTreeSet<Rc<String>>, usize), usize>;

fn calc_max_pressure_release_rec2(
    valves: &HashMap<Rc<String>, (usize, Vec<Rc<String>>)>,
    remaining_time: usize,
    my_pos: Rc<String>,
    elephant_pos: Rc<String>,
    used: BTreeSet<Rc<String>>,
    cache: &mut PairCache,
) -> usize {
    if let Some(existing) = cache.get_mut(&(
        my_pos.clone(),
        elephant_pos.clone(),
        used.clone(),
        remaining_time,
    )) {
        return *existing;
    }
    let new_value = used.iter().map(|x| valves.get(x).unwrap().0).sum::<usize>();
    if remaining_time == 0 {
        return 0;
    }
    let mut max_sum = 0;
    if let Some((value, neighbours)) = valves.get(&my_pos) {
        if let Some((el_value, el_neighbours)) = valves.get(&elephant_pos) {
            let mut neighbours = neighbours.clone();
            let mut el_neighbours = el_neighbours.clone();
            if *value > 0 && !used.contains(&my_pos) {
                neighbours.push(my_pos.clone());
            }
            if *el_value > 0 && !used.contains(&elephant_pos) {
                el_neighbours.push(elephant_pos.clone());
            }
            let possible_steps_iter = neighbours.iter().cartesian_product(el_neighbours.iter());
            max_sum = max_sum.max(
                possible_steps_iter
                    .map(|(my_new_pos, el_new_pos)| {
                        let mut new_used = used.clone();
                        if my_new_pos == &my_pos {
                            new_used.insert(my_pos.clone());
                        }
                        if el_new_pos == &elephant_pos {
                            new_used.insert(elephant_pos.clone());
                        }
                        calc_max_pressure_release_rec2(
                            valves,
                            remaining_time - 1,
                            my_new_pos.clone(),
                            el_new_pos.clone(),
                            new_used,
                            cache,
                        )
                    })
                    .max()
                    .unwrap_or(0),
            );
        }
    }
    let new_sum = new_value + max_sum;
    cache.insert((my_pos, elephant_pos, used, remaining_time), new_sum);
    new_sum
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    item: Rc<String>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.item.cmp(&other.item))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_distance_between_valued_nodes(
    valves: &HashMap<Rc<String>, (usize, Vec<Rc<String>>)>,
) -> HashMap<Rc<String>, (usize, HashMap<Rc<String>, usize>)> {
    let mut dist: HashMap<Rc<String>, (usize, HashMap<Rc<String>, usize>)> = HashMap::new();

    let items_to_track = valves
        .iter()
        .filter_map(|(k, (v, _))| {
            if *v > 0 || k.starts_with("AA") {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    for (valve, (amount, _)) in valves {
        let mut heap = BinaryHeap::new();

        heap.push(State {
            cost: 0,
            item: valve.clone(),
        });
        dist.entry(valve.clone())
            .or_insert((*amount, HashMap::new()))
            .1
            .entry(valve.clone())
            .or_insert(0);
        for _ in valves {
            while let Some(State { cost, item }) = heap.pop() {
                if cost
                    > *dist
                        .entry(valve.clone())
                        .or_insert((*amount, HashMap::new()))
                        .1
                        .entry(item.clone())
                        .or_insert(usize::MAX)
                {
                    continue;
                }
                for next in &valves.get(&item).unwrap().1 {
                    let next_state = State {
                        cost: cost + 1,
                        item: next.clone(),
                    };

                    if next_state.cost
                        < *dist
                            .entry(valve.clone())
                            .or_insert((*amount, HashMap::new()))
                            .1
                            .entry(next_state.item.clone())
                            .or_insert(usize::MAX)
                    {
                        *dist
                            .entry(valve.clone())
                            .or_insert((*amount, HashMap::new()))
                            .1
                            .entry(next_state.item.clone())
                            .or_insert(usize::MAX) = next_state.cost;
                        heap.push(next_state);
                    }
                }
            }
        }
    }

    dist.retain(|k, _| items_to_track.contains(k));
    dist.iter_mut().for_each(|(_, (_, v))| {
        v.retain(|k, _| items_to_track.contains(k));
    });

    dist
}

fn parse(filename: &str) -> HashMap<Rc<String>, (usize, Vec<Rc<String>>)> {
    let re = regex::Regex::new(
        r#"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#,
    )
    .unwrap();
    read_to_string(filename)
        .expect("failed to read file")
        .trim()
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                Rc::new(caps[1].into()),
                (
                    caps[2].parse().unwrap(),
                    caps[3]
                        .trim()
                        .split_terminator(',')
                        .map(|s| Rc::new(s.trim().into()))
                        .collect(),
                ),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let valves = parse("test-input.txt");
        assert_eq!(calc_max_pressure_release(&valves), 1651);
    }

    #[test]
    fn part2_test() {
        let valves = parse("test-input.txt");
        assert_eq!(calc_max_pressure_release2(&valves), 1707);
    }
}
