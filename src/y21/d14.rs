use std::collections::HashMap;
use std::ops::AddAssign;
use std::str::FromStr;

use itertools::{Itertools, MinMaxResult};

type Rules = HashMap<(char, char), char>;

#[derive(Debug, Clone, Default)]
struct Counts(HashMap<char, usize>);

impl AddAssign for Counts {
    fn add_assign(&mut self, cnts: Self) {
        cnts.0.iter().for_each(|(c, cnt)| {
            *self.0.entry(*c).or_insert(0) += cnt;
        });
    }
}

impl Counts {
    fn dec(&mut self, c: char) {
        self.0.entry(c).and_modify(|cnt| *cnt -= 1);
    }
}

#[derive(Debug, Clone)]
struct Polymer {
    poly: Vec<char>,
    rules: Rules,
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
    fn count_pair(
        &self,
        cache: &mut HashMap<((char, char), u64), Counts>,
        pair: (char, char),
        depth: u64,
    ) -> Counts {
        cache.get(&(pair, depth)).cloned().unwrap_or_else(|| {
            let cnts = if depth == 0 || !self.rules.contains_key(&pair) {
                Counts([pair.0, pair.1].iter().copied().counts())
            } else {
                let insert = *self.rules.get(&pair).unwrap();
                let mut cnts = Counts::default();
                cnts += self.count_pair(cache, (pair.0, insert), depth - 1);
                cnts += self.count_pair(cache, (insert, pair.1), depth - 1);
                cnts.dec(insert);
                cnts
            };
            cache.insert((pair, depth), cnts.clone());
            cnts
        })
    }
}

fn run_poly(poly: &Polymer, n: u64) -> Result<usize, String> {
    let mut cache = HashMap::new();
    let counts = poly
        .poly
        .iter()
        .tuple_windows()
        .fold(Counts::default(), |mut cnts, (c1, c2)| {
            cnts += poly.count_pair(&mut cache, (*c1, *c2), n);
            cnts.dec(*c1);
            cnts
        });
    if let MinMaxResult::MinMax(min, max) = counts.0.values().minmax() {
        Ok(max - min)
    } else {
        Err("Couldn't find distinct min and max".into())
    }
}

fn part1(poly: &Polymer) -> Result<usize, String> {
    run_poly(poly, 10)
}

fn part2(poly: &Polymer) -> Result<usize, String> {
    run_poly(poly, 40)
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d14.txt");
    let poly = input.parse::<Polymer>()?;
    let out1 = part1(&poly)?;
    let out2 = part2(&poly)?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part1(&poly), Ok(1588));
    }

    #[test]
    fn test02() {
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
        assert_eq!(part2(&poly), Ok(2_188_189_693_529));
    }
}
