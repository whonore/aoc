use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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
    fn identify(&self) -> Option<u8> {
        match self.0.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
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

fn part1(runs: &[(Vec<Digit>, Vec<Digit>)]) -> usize {
    runs.iter()
        .flat_map(|(_, outs)| outs.iter().filter_map(Digit::identify))
        .count()
}

fn part2() -> u64 {
    0
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
    let out2 = part2();
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
}
