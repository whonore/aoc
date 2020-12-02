use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Mode {
    MinMax,
    PosXor,
}
use Mode::*;

#[derive(PartialEq, Eq, Debug)]
struct Policy {
    mode: Mode,
    letter: char,
    n1: usize,
    n2: usize,
}

impl FromStr for Policy {
    type Err = String;

    fn from_str(policy: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = policy.split(' ').collect();
        let range: Vec<&str> = fields[0].split('-').collect();
        Ok(Self {
            mode: MinMax,
            letter: fields[1].parse().map_err(|_| "Invalid letter")?,
            n1: range[0].parse().map_err(|_| "Invalid n1")?,
            n2: range[1].parse().map_err(|_| "Invalid n2")?,
        })
    }
}

impl Policy {
    fn swap_mode(&mut self) {
        self.mode = match self.mode {
            MinMax => PosXor,
            PosXor => MinMax,
        }
    }

    fn is_valid(&self, pass: &str) -> bool {
        match self.mode {
            MinMax => {
                let n = pass.matches(self.letter).count();
                self.n1 <= n && n <= self.n2
            }
            PosXor => {
                let c1 = pass.chars().nth(self.n1 - 1);
                let c2 = pass.chars().nth(self.n2 - 1);
                (c1 == Some(self.letter)) ^ (c2 == Some(self.letter))
            }
        }
    }
}

fn solve(xs: &[(Policy, &str)]) -> usize {
    xs.iter()
        .filter(|(policy, pass)| policy.is_valid(pass))
        .count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p02.txt");
    let mut xs: Vec<(Policy, &str)> = input
        .lines()
        .map(|x| {
            let x: Vec<&str> = x.split(": ").collect();
            (x[0].parse().unwrap(), x[1])
        })
        .collect();
    let out1 = solve(&xs);
    xs.iter_mut().for_each(|(policy, _)| policy.swap_mode());
    let out2 = solve(&xs);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            ("1-3 a").parse(),
            Ok(Policy {
                mode: MinMax,
                letter: 'a',
                n1: 1,
                n2: 3
            })
        );
    }

    #[test]
    fn test_valid() {
        let mut p = Policy {
            mode: MinMax,
            letter: 'a',
            n1: 1,
            n2: 3,
        };
        assert!(!p.is_valid(""));
        assert!(p.is_valid("a"));
        assert!(p.is_valid("aa"));
        assert!(p.is_valid("aaa"));
        assert!(!p.is_valid("aaaa"));
        p.swap_mode();
        assert!(!p.is_valid(""));
        assert!(p.is_valid("a"));
        assert!(p.is_valid("bba"));
        assert!(!p.is_valid("aba"));
        assert!(!p.is_valid("bbb"));
    }

    #[test]
    fn test01() {
        assert_eq!(
            solve(&[
                (
                    Policy {
                        mode: MinMax,
                        letter: 'a',
                        n1: 1,
                        n2: 3
                    },
                    "abcde".into()
                ),
                (
                    Policy {
                        mode: MinMax,
                        letter: 'b',
                        n1: 1,
                        n2: 3
                    },
                    "cdefg".into()
                ),
                (
                    Policy {
                        mode: MinMax,
                        letter: 'c',
                        n1: 2,
                        n2: 9
                    },
                    "ccccccccc".into()
                )
            ]),
            2
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            solve(&[
                (
                    Policy {
                        mode: PosXor,
                        letter: 'a',
                        n1: 1,
                        n2: 3
                    },
                    "abcde".into()
                ),
                (
                    Policy {
                        mode: PosXor,
                        letter: 'b',
                        n1: 1,
                        n2: 3
                    },
                    "cdefg".into()
                ),
                (
                    Policy {
                        mode: PosXor,
                        letter: 'c',
                        n1: 2,
                        n2: 9
                    },
                    "ccccccccc".into()
                )
            ]),
            1
        );
    }
}
