use itertools::*;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let cubes = parse("input.txt");
    println!("part1 solution {}", count_surface_area(&cubes));
    println!("part2 solution {}", count_exterior_surface_area(&cubes));
}

fn count_surface_area(cubes: &[(isize, isize, isize)]) -> usize {
    let groups = group_cubes(cubes);

    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|item| {
                    [
                        (1, 0, 0),
                        (-1, 0, 0),
                        (0, 1, 0),
                        (0, -1, 0),
                        (0, 0, -1),
                        (0, 0, 1),
                    ]
                    .iter()
                    .filter(|diff| {
                        !group.contains(&(item.0 + diff.0, item.1 + diff.1, item.2 + diff.2))
                    })
                    .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn count_exterior_surface_area(cubes: &[(isize, isize, isize)]) -> usize {
    let (min_x, max_x, min_y, max_y, min_z, max_z, lava_fields) = cubes.iter().fold(
        (
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            HashSet::new(),
        ),
        |mut acc, item| {
            acc.6.insert(*item);
            (
                acc.0.min(item.0),
                acc.1.max(item.0),
                acc.2.min(item.1),
                acc.3.max(item.1),
                acc.4.min(item.2),
                acc.5.max(item.2),
                acc.6,
            )
        },
    );
    let mut air_fields = HashSet::new();
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                if !lava_fields.contains(&(x, y, z)) {
                    air_fields.insert((x, y, z));
                }
            }
        }
    }

    let mut exterior = HashSet::new();
    let mut q = vec![(min_x - 1, min_y - 1, min_z - 1)];
    while let Some(item) = q.pop() {
        exterior.insert(item);
        for diff in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        {
            let (x, y, z) = (item.0 + diff.0, item.1 + diff.1, item.2 + diff.2);
            if x <= max_x + 1
                && x >= min_x - 1
                && y <= max_y + 1
                && y >= min_y - 1
                && z <= max_z + 1
                && z >= min_z - 1
                && air_fields.contains(&(x, y, z))
                && !exterior.contains(&(x, y, z))
            {
                q.push((x, y, z));
            }
        }
    }

    cubes
        .iter()
        .map(|item| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ]
            .iter()
            .filter(|diff| exterior.contains(&(item.0 + diff.0, item.1 + diff.1, item.2 + diff.2)))
            .count()
        })
        .sum()
}

fn group_cubes(cubes: &[(isize, isize, isize)]) -> Vec<HashSet<(isize, isize, isize)>> {
    let mut groups: Vec<HashSet<(isize, isize, isize)>> = vec![];
    for cube in cubes {
        if groups
            .iter()
            .any(|group| group.iter().any(|cube2| manhattan_dist(cube, cube2) <= 1))
        {
            groups
                .iter_mut()
                .filter(|group| group.iter().any(|cube2| manhattan_dist(cube, cube2) <= 1))
                .for_each(|group| {
                    group.insert(*cube);
                });
        } else {
            groups.push(HashSet::from_iter(vec![*cube].into_iter()));
        }
    }
    loop {
        let cartesian_prod_iter = (0..groups.len()).cartesian_product(0..groups.len());
        if cartesian_prod_iter
            .map(|(i, j)| {
                if i != j
                    && groups[i].iter().any(|item| {
                        groups[j]
                            .iter()
                            .any(|item2| manhattan_dist(item, item2) <= 1)
                    })
                {
                    let group_to_move = std::mem::take(&mut groups[j]);
                    groups[i].extend(group_to_move.into_iter());
                    1
                } else {
                    0
                }
            })
            .sum::<usize>()
            == 0
        {
            break;
        }
        groups.retain(|group| !group.is_empty());
    }

    groups
}

fn manhattan_dist(a: &(isize, isize, isize), b: &(isize, isize, isize)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()).unsigned_abs()
}

fn parse(filename: &str) -> Vec<(isize, isize, isize)> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| {
            let values = l
                .split_terminator(',')
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>();
            (values[0], values[1], values[2])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let cubes = vec![(1, 1, 1), (2, 1, 1)];
        assert_eq!(count_surface_area(&cubes), 10);
    }

    #[test]
    fn part1_test2() {
        let cubes = parse("test-input.txt");
        assert_eq!(count_surface_area(&cubes), 64);
    }

    #[test]
    fn part2_test() {
        let cubes = parse("test-input.txt");
        assert_eq!(count_exterior_surface_area(&cubes), 58);
    }
}
