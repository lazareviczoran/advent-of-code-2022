use std::{cmp::Ordering, fs::read_to_string, iter::Peekable, str::Chars};

fn main() {
    let pairs = parse("input.txt");
    println!("part1 solution {}", sum_pos_of_ordered(&pairs));
    println!("part2 solution {:?}", find_decoder_key(pairs));
}

#[derive(Debug, Clone)]
enum Type {
    Vec(Vec<Type>),
    Int(isize),
}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}
impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Type {}
impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Type::Vec(l), Type::Vec(r)) => {
                if r.is_empty() && l.len() > r.len() {
                    return Ordering::Greater;
                } else if l.is_empty() && l.len() < r.len() {
                    return Ordering::Less;
                }
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                loop {
                    match (l_iter.next(), r_iter.next()) {
                        (Some(l), Some(r)) => match l.cmp(r) {
                            Ordering::Equal => continue,
                            ordering => return ordering,
                        },
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (Type::Int(l), Type::Vec(r)) => {
                Type::Vec(vec![Type::Int(*l)]).cmp(&Type::Vec(r.to_vec()))
            }
            (Type::Vec(l), Type::Int(r)) => {
                Type::Vec(l.to_vec()).cmp(&Type::Vec(vec![Type::Int(*r)]))
            }
            (Type::Int(l), Type::Int(r)) => l.cmp(r),
        }
    }
}

fn sum_pos_of_ordered(pairs: &[(Type, Type)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
        .sum()
}

fn find_decoder_key(pairs: Vec<(Type, Type)>) -> Option<usize> {
    let mut packets = pairs
        .into_iter()
        .flat_map(|(l, r)| vec![l, r])
        .collect::<Vec<_>>();
    let divider1 = Type::Vec(vec![Type::Vec(vec![Type::Int(2)])]);
    let divider2 = Type::Vec(vec![Type::Vec(vec![Type::Int(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let pos1 = packets.iter().position(|p| p == &divider1)?;
    let pos2 = packets.iter().position(|p| p == &divider2)?;
    Some((pos1 + 1) * (pos2 + 1))
}

fn parse_str(chars_iter: &mut Peekable<Chars>) -> Option<Type> {
    while let Some(ch) = chars_iter.peek() {
        match ch {
            '[' => {
                chars_iter.next();
                let mut vec = Vec::new();
                while let Some(item) = parse_str(chars_iter) {
                    vec.push(item);
                }
                return Some(Type::Vec(vec));
            }
            a if a.is_numeric() => {
                let mut value = String::new();
                for next_ch in chars_iter.by_ref() {
                    if !next_ch.is_numeric() {
                        break;
                    }
                    value.push(next_ch);
                }
                return Some(Type::Int(value.parse().ok()?));
            }
            ']' | ',' => {
                chars_iter.next();
                return None;
            }
            invalid => panic!("unexpected '{invalid}'"),
        }
    }
    None
}

fn parse(filename: &str) -> Vec<(Type, Type)> {
    let content = read_to_string(filename).expect("failed to read file");
    content
        .split_terminator("\n\n")
        .filter_map(|pair| {
            pair.split_once('\n').and_then(|(left, right)| {
                Some((
                    parse_str(&mut left.trim().chars().peekable())?,
                    parse_str(&mut right.trim().chars().peekable())?,
                ))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let pairs = parse("test-input.txt");
        assert_eq!(sum_pos_of_ordered(&pairs), 13);
    }

    #[test]
    fn part2_test() {
        let pairs = parse("test-input.txt");
        assert_eq!(find_decoder_key(pairs), Some(140));
    }
}
