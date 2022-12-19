use std::{
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
};

fn main() {
    // let valves = parse("input.txt");
    let valves = parse("test-input.txt");
    println!(
        "part1 solution {}",
        calc_max_pressure_release(&valves, "AA".into())
    );
}

fn calc_max_pressure_release(
    valves: &HashMap<String, (usize, Vec<String>)>,
    start: String,
) -> usize {
    let remaining_unused = valves
        .iter()
        .filter_map(|(item, &(value, _))| if value > 0 { Some(item.clone()) } else { None })
        .collect::<BTreeSet<String>>();
    calc_max_pressure_release_rec(
        valves,
        30,
        start,
        0,
        remaining_unused,
        BTreeSet::new(),
        &mut HashMap::new(),
    )
}

fn calc_max_pressure_release_rec(
    valves: &HashMap<String, (usize, Vec<String>)>,
    remaining_time: usize,
    curr_valve: String,
    curr_sum: usize,
    remaining_unused: BTreeSet<String>,
    used: BTreeSet<String>,
    cache: &mut HashMap<(BTreeSet<String>, usize), usize>,
) -> usize {
    println!("remaining {remaining_time}, curr {curr_valve}, unused {remaining_unused:?}");
    println!("curr sum {curr_sum}");
    // if let Some(existing) = cache.get_mut(&(used.clone(), remaining_time)) {
    //     // *existing += curr_sum;
    //     return *existing;
    // }
    if remaining_time == 0 || remaining_unused.is_empty() {
        return curr_sum;
    }
    let new_value = used.iter().map(|x| valves.get(x).unwrap().0).sum::<usize>();
    let mut max_sum = 0;
    if let Some((value, neighbours)) = valves.get(&curr_valve) {
        if *value > 0 && remaining_unused.contains(&curr_valve) {
            println!("open {curr_valve}");
            max_sum = max_sum.max(calc_max_pressure_release_rec(
                valves,
                remaining_time - 1,
                curr_valve.clone(),
                curr_sum + value,
                &remaining_unused - &BTreeSet::from_iter(vec![curr_valve.clone()].into_iter()),
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
                        curr_sum,
                        remaining_unused.clone(),
                        used.clone(),
                        cache,
                    )
                })
                .max()
                .unwrap_or(0),
        );
    }
    let new_sum = new_value + curr_sum + max_sum;
    cache.insert((used, remaining_time), new_sum);
    new_sum
}

fn parse(filename: &str) -> HashMap<String, (usize, Vec<String>)> {
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
                caps[1].into(),
                (
                    caps[2].parse().unwrap(),
                    caps[3]
                        .trim()
                        .split_terminator(',')
                        .map(|s| s.trim().into())
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
        println!("{valves:?}");
        assert_eq!(calc_max_pressure_release(&valves, "AA".into()), 1651);
    }
}
