use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Seg {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Seg {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(Self::A),
            'b' => Some(Self::B),
            'c' => Some(Self::C),
            'd' => Some(Self::D),
            'e' => Some(Self::E),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Digit(Vec<Seg>);

impl Digit {
    fn from_digit(x: u8) -> Option<Self> {
        match x {
            0 => Some(Self(vec![Seg::A, Seg::B, Seg::C, Seg::E, Seg::F, Seg::G])),
            1 => Some(Self(vec![Seg::C, Seg::F])),
            2 => Some(Self(vec![Seg::A, Seg::C, Seg::D, Seg::E, Seg::G])),
            3 => Some(Self(vec![Seg::A, Seg::C, Seg::D, Seg::F, Seg::G])),
            4 => Some(Self(vec![Seg::B, Seg::C, Seg::D, Seg::F])),
            5 => Some(Self(vec![Seg::A, Seg::B, Seg::D, Seg::F, Seg::G])),
            6 => Some(Self(vec![Seg::A, Seg::B, Seg::D, Seg::E, Seg::F, Seg::G])),
            7 => Some(Self(vec![Seg::A, Seg::C, Seg::F])),
            8 => Some(Self(vec![
                Seg::A,
                Seg::B,
                Seg::C,
                Seg::D,
                Seg::E,
                Seg::F,
                Seg::G,
            ])),
            9 => Some(Self(vec![Seg::A, Seg::B, Seg::C, Seg::D, Seg::F, Seg::G])),
            _ => None,
        }
    }

    fn to_digit(&self) -> Option<u8> {
        let mut segs = self.0.clone();
        segs.sort_unstable();
        match segs.as_slice() {
            [Seg::A, Seg::B, Seg::C, Seg::E, Seg::F, Seg::G] => Some(0),
            [Seg::C, Seg::F] => Some(1),
            [Seg::A, Seg::C, Seg::D, Seg::E, Seg::G] => Some(2),
            [Seg::A, Seg::C, Seg::D, Seg::F, Seg::G] => Some(3),
            [Seg::B, Seg::C, Seg::D, Seg::F] => Some(4),
            [Seg::A, Seg::B, Seg::D, Seg::F, Seg::G] => Some(5),
            [Seg::A, Seg::B, Seg::D, Seg::E, Seg::F, Seg::G] => Some(6),
            [Seg::A, Seg::C, Seg::F] => Some(7),
            [Seg::A, Seg::B, Seg::C, Seg::D, Seg::E, Seg::F, Seg::G] => Some(8),
            [Seg::A, Seg::B, Seg::C, Seg::D, Seg::F, Seg::G] => Some(9),
            _ => None,
        }
    }

    fn identify_unique(&self) -> Option<u8> {
        match self.0.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn identify(&self) -> Vec<u8> {
        match self.0.len() {
            5 => vec![2, 3, 5],
            6 => vec![0, 6, 9],
            _ => self.identify_unique().iter().copied().collect(),
        }
    }
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(segs: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            segs.chars()
                .map(Seg::from_char)
                .collect::<Option<Vec<_>>>()
                .ok_or("Invalid segment")?,
        ))
    }
}

#[derive(Debug, Clone)]
struct Mapping(HashMap<Seg, HashSet<Seg>>);

impl Mapping {
    fn new() -> Self {
        let segs = [Seg::A, Seg::B, Seg::C, Seg::D, Seg::E, Seg::F, Seg::G];
        Self(
            segs.iter()
                .copied()
                .map(|seg| (seg, segs.iter().copied().collect()))
                .collect(),
        )
    }

    fn is_decoded(&self) -> bool {
        self.0.values().all(|segs| segs.len() == 1)
    }

    fn get(&self, seg: Seg) -> Seg {
        self.0[&seg].iter().copied().next().unwrap()
    }

    fn add_option(&mut self, seg: Seg, dig: &[Seg]) {
        let dig = dig.iter().copied().collect::<HashSet<_>>();
        self.0.entry(seg).and_modify(|digs| {
            *digs = digs.intersection(&dig).copied().collect();
        });
    }

    fn normalize(&mut self) {
        let unique = self
            .0
            .iter()
            .filter_map(|(&seg, segs)| {
                (segs.len() == 1).then(|| (seg, segs.iter().copied().next().unwrap()))
            })
            .collect::<Vec<_>>();
        for (&seg, segs) in &mut self.0 {
            for (useg, umap) in &unique {
                if seg != *useg {
                    segs.remove(umap);
                }
            }
        }
    }

    fn valid(&self) -> bool {
        self.0.values().all(|segs| !segs.is_empty())
    }
}

// TODO: this is very ugly
fn decode(pats: &[Digit], out: &[Digit]) -> u64 {
    let decode_digit = |m: &Mapping, dig: &Digit| -> Option<u8> {
        Digit(dig.0.iter().map(|&seg| m.get(seg)).collect::<Vec<_>>()).to_digit()
    };
    let mapping_ok = |m: &Mapping| -> bool {
        m.is_decoded() && out.iter().all(|dig| decode_digit(m, dig).is_some())
    };

    let mut mappings = vec![Mapping::new()];
    let options = pats
        .iter()
        .chain(out.iter())
        .map(|pat| {
            (
                pat,
                pat.identify()
                    .iter()
                    .map(|&dig| Digit::from_digit(dig).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    for (pat, digs) in &options {
        if digs.len() == 1 {
            for dig in digs {
                for &seg in &pat.0 {
                    mappings[0].add_option(seg, &dig.0);
                    mappings[0].normalize();
                }
            }
        }
    }

    let mapping = loop {
        for (pat, digs) in &options {
            if digs.len() == 1 {
                continue;
            }
            for dig in digs {
                mappings = mappings
                    .iter()
                    .flat_map(|mapping| {
                        pat.0.iter().map(move |&seg| {
                            let mut m = mapping.clone();
                            m.add_option(seg, &dig.0);
                            m.normalize();
                            m
                        })
                    })
                    .filter(Mapping::valid)
                    .collect();
            }
            if mappings.iter().any(mapping_ok) {
                break;
            }
        }
        if let Some(m) = mappings.iter().find(|m| mapping_ok(m)) {
            break m;
        }
    };

    out.iter()
        .map(|dig| decode_digit(mapping, dig).unwrap())
        .fold(0, |sum, dig| sum * 10 + u64::from(dig))
}

fn part1(runs: &[(Vec<Digit>, Vec<Digit>)]) -> usize {
    runs.iter()
        .flat_map(|(_, outs)| outs.iter().filter_map(Digit::identify_unique))
        .count()
}

fn part2(runs: &[(Vec<Digit>, Vec<Digit>)]) -> u64 {
    runs.iter().map(|(pats, out)| decode(pats, out)).sum()
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d08.txt");
    let runs = input
        .lines()
        .map(|line| -> Result<(Vec<Digit>, Vec<Digit>), String> {
            let mut line = line.split(" | ");
            let pats = line.next().unwrap();
            let out = line.next().unwrap();
            Ok((
                pats.split_whitespace()
                    .map(str::parse)
                    .collect::<Result<_, _>>()?,
                out.split_whitespace()
                    .map(str::parse)
                    .collect::<Result<_, _>>()?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&runs);
    let out2 = part2(&runs);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let runs = [
            (
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb",
                "fdgacbe cefdb cefbgd gcbe",
            ),
            (
                "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec",
                "fcgedb cgb dgebacf gc",
            ),
            (
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef",
                "cg cg fdcagb cbg",
            ),
            (
                "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega",
                "efabcd cedba gadfec cb",
            ),
            (
                "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga",
                "gecf egdcabf bgf bfgea",
            ),
            (
                "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf",
                "gebdcfa ecba ca fadegcb",
            ),
            (
                "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf",
                "cefg dcbef fcge gbcadfe",
            ),
            (
                "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd",
                "ed bcgafe cdgba cbgef",
            ),
            (
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg",
                "gbdfcae bgc cg cgb",
            ),
            (
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc",
                "fgae cfgab fg bagce",
            ),
        ];
        let runs = runs
            .iter()
            .map(|(pats, out)| -> Result<(Vec<Digit>, Vec<Digit>), String> {
                Ok((
                    pats.split_whitespace()
                        .map(str::parse)
                        .collect::<Result<_, _>>()?,
                    out.split_whitespace()
                        .map(str::parse)
                        .collect::<Result<_, _>>()?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(part1(&runs), 26);
    }

    #[test]
    fn test_decode() {
        let pats = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let out = "cdfeb fcadb cdfeb cdbaf"
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(decode(&pats, &out), 5353);
    }

    #[ignore]
    #[test]
    fn test02() {
        let runs = [
            (
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb",
                "fdgacbe cefdb cefbgd gcbe",
            ),
            (
                "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec",
                "fcgedb cgb dgebacf gc",
            ),
            (
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef",
                "cg cg fdcagb cbg",
            ),
            (
                "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega",
                "efabcd cedba gadfec cb",
            ),
            (
                "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga",
                "gecf egdcabf bgf bfgea",
            ),
            (
                "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf",
                "gebdcfa ecba ca fadegcb",
            ),
            (
                "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf",
                "cefg dcbef fcge gbcadfe",
            ),
            (
                "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd",
                "ed bcgafe cdgba cbgef",
            ),
            (
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg",
                "gbdfcae bgc cg cgb",
            ),
            (
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc",
                "fgae cfgab fg bagce",
            ),
        ];
        let runs = runs
            .iter()
            .map(|(pats, out)| -> Result<(Vec<Digit>, Vec<Digit>), String> {
                Ok((
                    pats.split_whitespace()
                        .map(str::parse)
                        .collect::<Result<_, _>>()?,
                    out.split_whitespace()
                        .map(str::parse)
                        .collect::<Result<_, _>>()?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            runs.iter()
                .map(|(pats, out)| decode(&pats, &out))
                .collect::<Vec<_>>(),
            vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315]
        );
        assert_eq!(part2(&runs), 61229);
    }
}
