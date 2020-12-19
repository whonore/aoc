use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Ref(u32),
    Concat(Vec<Rule>),
    Alt(Vec<Rule>),
}
use Rule::*;

impl FromStr for Rule {
    type Err = String;

    fn from_str(rule: &str) -> Result<Self, Self::Err> {
        Ok(Alt(rule
            .split(" | ")
            .map(|rule| {
                if rule.starts_with('"') {
                    Char(rule.chars().nth(1).unwrap())
                } else {
                    Concat(
                        rule.split_whitespace()
                            .map(|r| Ref(r.parse::<u32>().unwrap()))
                            .collect(),
                    )
                }
            })
            .collect()))
    }
}

#[derive(Debug)]
struct Rules(HashMap<u32, Rule>);

impl Rules {
    fn strip_match<'a>(&self, rule: &Rule, s: &'a str) -> Option<&'a str> {
        match rule {
            Char(c) => s.strip_prefix(*c),
            Ref(r) => self.strip_match(&self.0[&r], s),
            Concat(rs) => rs.iter().try_fold(s, |rest, r| self.strip_match(r, rest)),
            Alt(rs) => rs.iter().find_map(|r| self.strip_match(r, s)),
        }
    }

    fn matches(&self, rule: u32, s: &str) -> bool {
        self.strip_match(&self.0[&rule], s)
            .map(|remain| remain.is_empty())
            .unwrap_or(false)
    }
}

impl FromStr for Rules {
    type Err = String;

    fn from_str(rules: &str) -> Result<Self, Self::Err> {
        Ok(Rules(
            rules
                .lines()
                .map(|line| {
                    let line = line.split(": ").collect::<Vec<_>>();
                    let n = line[0].parse::<u32>().unwrap();
                    let rule = line[1].parse::<Rule>()?;
                    Ok((n, rule))
                })
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}

fn solve(rules: &Rules, msgs: &[&str]) -> usize {
    msgs.iter().filter(|msg| rules.matches(0, msg)).count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p19.txt");
    let blocks = input.split("\n\n").collect::<Vec<_>>();
    let rules = blocks[0].parse::<Rules>()?;
    let msgs = blocks[1].lines().collect::<Vec<_>>();
    let out1 = solve(&rules, &msgs);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let rules = Rules(
            [
                (0, Concat(vec![Ref(4), Ref(1), Ref(5)])),
                (
                    1,
                    Alt(vec![
                        Concat(vec![Ref(2), Ref(3)]),
                        Concat(vec![Ref(3), Ref(2)]),
                    ]),
                ),
                (
                    2,
                    Alt(vec![
                        Concat(vec![Ref(4), Ref(4)]),
                        Concat(vec![Ref(5), Ref(5)]),
                    ]),
                ),
                (
                    3,
                    Alt(vec![
                        Concat(vec![Ref(4), Ref(5)]),
                        Concat(vec![Ref(5), Ref(4)]),
                    ]),
                ),
                (4, Char('a')),
                (5, Char('b')),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        let msgs = ["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"];
        assert_eq!(solve(&rules, &msgs), 2);
    }
}
