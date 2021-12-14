use std::collections::HashMap;
use std::str::FromStr;

use itertools::{Itertools, MinMaxResult};

#[derive(Debug, Clone)]
struct Polymer {
    poly: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(poly: &str) -> Result<Self, Self::Err> {
        let (poly, rules) = {
            let mut lines = poly.split("\n\n");
            (lines.next().unwrap(), lines.next().unwrap())
        };
        Ok(Self {
            poly: poly.chars().collect(),
            rules: rules
                .lines()
                .map(|rule| {
                    let (left, right) = {
                        let mut sides = rule.split(" -> ");
                        (sides.next().unwrap(), sides.next().unwrap())
                    };
                    (
                        (left.chars().next().unwrap(), left.chars().nth(1).unwrap()),
                        right.chars().next().unwrap(),
                    )
                })
                .collect(),
        })
    }
}

impl Polymer {
    fn apply(&mut self) {
        self.poly
            .iter()
            .enumerate()
            .tuple_windows()
            .filter_map(|((_, c1), (n, c2))| self.rules.get(&(*c1, *c2)).map(|insert| (n, *insert)))
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .for_each(|(idx, (n, insert))| {
                self.poly.insert(n + idx, *insert);
            });
    }
}

fn part1(mut poly: Polymer) -> Result<usize, String> {
    for _ in 0..10 {
        poly.apply();
    }
    if let MinMaxResult::MinMax(min, max) = poly.poly.iter().counts().values().minmax() {
        Ok(max - min)
    } else {
        Err("Couldn't find distinct min and max".into())
    }
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d14.txt");
    let poly = input.parse()?;
    let out1 = part1(poly)?;
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rules() {
        let mut poly = Polymer {
            poly: vec!['N', 'N', 'C', 'B'],
            rules: [
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]
            .iter()
            .copied()
            .collect(),
        };
        poly.apply();
        assert_eq!(poly.poly.iter().collect::<String>(), "NCNBCHB");
        poly.apply();
        assert_eq!(poly.poly.iter().collect::<String>(), "NBCCNBBBCBHCB");
        poly.apply();
        assert_eq!(
            poly.poly.iter().collect::<String>(),
            "NBBBCNCCNBBNBNBBCHBHHBCHB"
        );
        poly.apply();
        assert_eq!(
            poly.poly.iter().collect::<String>(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
    }

    #[test]
    fn test01() {
        let poly = Polymer {
            poly: vec!['N', 'N', 'C', 'B'],
            rules: [
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]
            .iter()
            .copied()
            .collect(),
        };
        assert_eq!(part1(poly), Ok(1588));
    }
}
