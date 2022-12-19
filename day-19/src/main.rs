use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let input = parse("test-input.txt");
    // let input = parse("input.txt");
    println!("part1 solution {}", calculate_outcome(&input));
}

fn calculate_outcome(input: &BTreeMap<usize, BTreeMap<String, Vec<(String, usize)>>>) -> usize {
    input
        .iter()
        .map(|(blueprint_id, requirements)| {
            println!("blueprint {blueprint_id}");
            blueprint_id * find_max_open_geodes(requirements)
        })
        .sum()
}

fn find_max_open_geodes(requirements: &BTreeMap<String, Vec<(String, usize)>>) -> usize {
    let mut max_open_geodes = 0;
    find_max_open_geodes_rec(
        requirements,
        24,
        vec!["ore".into()],
        BTreeMap::new(),
        &mut max_open_geodes,
    );
    max_open_geodes
}

fn find_max_open_geodes_rec(
    requirements: &BTreeMap<String, Vec<(String, usize)>>,
    remaining_time: usize,
    workers: Vec<String>,
    balance: BTreeMap<String, usize>,
    max_open_geodes: &mut usize,
) {
    println!("remaining_time: {}", remaining_time);
    println!("max_open_geodes: {}", max_open_geodes);
    println!("balance: {:?}", balance);
    println!("workers: {:?}", workers);
    let mut new_balance: BTreeMap<String, usize> = balance;
    if remaining_time == 0 {
        *max_open_geodes = (*max_open_geodes).max(*new_balance.get("aageode").unwrap_or(&0));
        return;
    }
    for (material, requirement_vec) in requirements {
        let mut new_workers = workers.clone();
        if requirement_vec
            .iter()
            .all(|(key, amount)| new_balance.get(key).unwrap_or(&0) >= amount)
        {
            new_workers.push(material.clone());
            requirement_vec
                .iter()
                .for_each(|(key, amount)| *new_balance.get_mut(key).unwrap() -= amount);
            find_max_open_geodes_rec(
                requirements,
                remaining_time - 1,
                new_workers.clone(),
                new_balance.clone(),
                max_open_geodes,
            );
            new_workers.pop();
            requirement_vec
                .iter()
                .for_each(|(key, amount)| *new_balance.get_mut(key).unwrap() += amount);
        }
    }
    for worker in workers.iter() {
        *new_balance.entry(worker.clone()).or_insert(0) += 1;
    }
    println!("new_balance: {:?}", new_balance);

    find_max_open_geodes_rec(
        requirements,
        remaining_time - 1,
        workers,
        new_balance.clone(),
        max_open_geodes,
    );
}

fn parse(filename: &str) -> BTreeMap<usize, BTreeMap<String, Vec<(String, usize)>>> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .filter_map(|line| {
            let mut requirements = BTreeMap::new();
            let content = line.strip_prefix("Blueprint ")?;
            let (blueprint_id, content) = content.split_once(':')?;
            let content = content.strip_prefix(" Each ore robot costs ")?;
            let (ore_robot_ore_amount, content) = content.split_once(' ')?;
            requirements.insert(
                "ore".into(),
                vec![("ore".into(), ore_robot_ore_amount.parse().ok()?)],
            );
            let content = content.strip_prefix("ore. Each clay robot costs ")?;
            let (clay_robot_ore_amount, content) = content.split_once(' ')?;
            requirements.insert(
                "clay".into(),
                vec![("ore".into(), clay_robot_ore_amount.parse().ok()?)],
            );
            let content = content.strip_prefix("ore. Each obsidian robot costs ")?;
            let (obsidian_robot_ore_amount, content) = content.split_once(' ')?;
            let content = content.strip_prefix("ore and ")?;
            let (obsidian_robot_clay_amount, content) = content.split_once(' ')?;
            let content = content.strip_prefix("clay. Each geode robot costs ")?;
            requirements.insert(
                "obsidian".into(),
                vec![
                    ("ore".into(), obsidian_robot_ore_amount.parse().ok()?),
                    ("clay".into(), obsidian_robot_clay_amount.parse().ok()?),
                ],
            );
            let (geode_robot_ore_amount, content) = content.split_once(' ')?;
            let geode_robot_obsidian_amount = content
                .strip_prefix("ore and ")?
                .strip_suffix(" obsidian.")?;
            requirements.insert(
                "aageode".into(),
                vec![
                    ("ore".into(), geode_robot_ore_amount.parse().ok()?),
                    ("obsidian".into(), geode_robot_obsidian_amount.parse().ok()?),
                ],
            );
            Some((blueprint_id.parse().ok()?, requirements))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = parse("test-input.txt");
        println!("{input:?}");
        assert_eq!(calculate_outcome(&input), 33);
    }
}
