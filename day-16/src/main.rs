use std::{
    collections::{HashMap, HashSet},
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
    let mut max_pressure = 0;
    let mut remaining_unused = valves
        .iter()
        .filter_map(|(item, &(value, _))| if value > 0 { Some(item.clone()) } else { None })
        .collect::<HashSet<String>>();
    calc_max_pressure_release_rec(
        valves,
        30,
        start,
        0,
        &mut max_pressure,
        &mut remaining_unused,
        &mut HashMap::new(),
    );
    max_pressure
}

fn calc_max_pressure_release_rec(
    valves: &HashMap<String, (usize, Vec<String>)>,
    remaining_time: usize,
    curr_valve: String,
    curr_sum: usize,
    max_sum: &mut usize,
    remaining_unused: &mut HashSet<String>,
    cache: &mut HashMap<usize, usize>,
) {
    println!("remaining {remaining_time}, curr {curr_valve}, unused {remaining_unused:?}");
    println!("curr sum {curr_sum}, max sum {max_sum}");
    if remaining_time == 0 || remaining_unused.is_empty() {
        *max_sum = (*max_sum).max(curr_sum);
        return;
    }
    if let Some((value, neighbours)) = valves.get(&curr_valve) {
        if *value > 0 && remaining_unused.contains(&curr_valve) {
            println!("minute {remaining_time} open {curr_valve}");
            remaining_unused.remove(&curr_valve);
            calc_max_pressure_release_rec(
                valves,
                remaining_time - 1,
                curr_valve.clone(),
                curr_sum + value * (remaining_time - 1),
                max_sum,
                remaining_unused,
                cache,
            );
            remaining_unused.insert(curr_valve.clone());
        }
        // calc_max_pressure_release_rec(
        //     valves,
        //     remaining_time - 1,
        //     curr_valve.clone(),
        //     curr_sum,
        //     max_sum,
        //     remaining_unused,
        //     cache,
        // );
        for neighbour in neighbours {
            calc_max_pressure_release_rec(
                valves,
                remaining_time - 1,
                neighbour.clone(),
                curr_sum,
                max_sum,
                remaining_unused,
                cache,
            );
        }
    }
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
        assert_eq!(calc_max_pressure_release(&valves, "AA".into()), 1);
    }
}
