use std::collections::HashMap;
use std::fmt;
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
        let mut rules = rule
            .split(" | ")
            .map(|rule| {
                if rule.starts_with('"') {
                    Char(rule.chars().nth(1).unwrap())
                } else {
                    let mut rules = rule
                        .split_whitespace()
                        .map(|r| Ref(r.parse::<u32>().unwrap()))
                        .collect::<Vec<_>>();
                    if rules.len() == 1 {
                        rules.remove(0)
                    } else {
                        Concat(rules)
                    }
                }
            })
            .collect::<Vec<_>>();
        Ok(if rules.len() == 1 {
            rules.remove(0)
        } else {
            Alt(rules)
        })
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Char(c) => write!(f, "{}", c),
            Ref(r) => write!(f, "{}", r),
            Concat(rs) => write!(
                f,
                "{}",
                rs.iter()
                    .map(|r| format!("{}", r))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Alt(rs) => write!(
                f,
                "{}",
                rs.iter()
                    .map(|r| format!("{}", r))
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
        }
    }
}

fn non_empty<A>(xs: Vec<A>) -> Option<Vec<A>> {
    if !xs.is_empty() {
        Some(xs)
    } else {
        None
    }
}

#[derive(Debug)]
struct Rules(HashMap<u32, Rule>);

impl Rules {
    fn strip_match<'a>(&self, rule: &Rule, s: &'a str) -> Option<Vec<&'a str>> {
        match rule {
            Char(c) => s.strip_prefix(*c).map(|rest| vec![rest]),
            Ref(r) => self.strip_match(&self.0[&r], s),
            Concat(rs) => non_empty(rs.iter().fold(vec![s], |rests, r| {
                rests
                    .iter()
                    .filter_map(|rest| self.strip_match(r, rest))
                    .flatten()
                    .collect()
            })),
            Alt(rs) => non_empty(
                rs.iter()
                    .filter_map(|r| self.strip_match(r, s))
                    .flatten()
                    .collect(),
            ),
        }
    }

    fn matches(&self, rule: u32, s: &str) -> bool {
        self.strip_match(&self.0[&rule], s)
            .map(|rests| rests.iter().any(|rest| rest.is_empty()))
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
    let mut rules = blocks[0].parse::<Rules>()?;
    let msgs = blocks[1].lines().collect::<Vec<_>>();
    let out1 = solve(&rules, &msgs);
    rules.0.insert(8, "42 | 42 8".parse::<Rule>()?);
    rules.0.insert(11, "42 31 | 42 11 31".parse::<Rule>()?);
    let out2 = solve(&rules, &msgs);
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

    #[test]
    fn test02() {
        let mut rules = "42: 9 14 | 10 1\n\
                         9: 14 27 | 1 26\n\
                         10: 23 14 | 28 1\n\
                         1: \"a\"\n\
                         11: 42 31\n\
                         5: 1 14 | 15 1\n\
                         19: 14 1 | 14 14\n\
                         12: 24 14 | 19 1\n\
                         16: 15 1 | 14 14\n\
                         31: 14 17 | 1 13\n\
                         6: 14 14 | 1 14\n\
                         2: 1 24 | 14 4\n\
                         0: 8 11\n\
                         13: 14 3 | 1 12\n\
                         15: 1 | 14\n\
                         17: 14 2 | 1 7\n\
                         23: 25 1 | 22 14\n\
                         28: 16 1\n\
                         4: 1 1\n\
                         20: 14 14 | 1 15\n\
                         3: 5 14 | 16 1\n\
                         27: 1 6 | 14 18\n\
                         14: \"b\"\n\
                         21: 14 1 | 1 14\n\
                         25: 1 1 | 1 14\n\
                         22: 14 14\n\
                         8: 42\n\
                         26: 14 22 | 1 20\n\
                         18: 15 15\n\
                         7: 14 5 | 1 21\n\
                         24: 14 1"
            .parse::<Rules>()
            .unwrap();
        let msgs = [
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        assert_eq!(solve(&rules, &msgs), 3);
        rules.0.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
        rules
            .0
            .insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());
        assert_eq!(solve(&rules, &msgs), 12);
    }

    #[test]
    fn test_loop() {
        let rules = "0: 1\n\
                     1: 2 | 2 1\n\
                     2: \"a\""
            .parse::<Rules>()
            .unwrap();
        assert!(rules.matches(0, "a"));
        assert!(rules.matches(0, "aa"));
        assert!(rules.matches(0, "aaa"));
        assert!(!rules.matches(0, "ab"));
        let rules = "0: 1\n\
                     1: 2 3 | 2 1 3\n\
                     2: \"a\"\n\
                     3: \"b\""
            .parse::<Rules>()
            .unwrap();
        assert!(rules.matches(0, "ab"));
        assert!(rules.matches(0, "aabb"));
        assert!(rules.matches(0, "aaabbb"));
        assert!(!rules.matches(0, "aab"));
        let rules = "0: 1 2\n\
                     1: 3 | 3 1\n\
                     2: 3 4 | 3 2 4\n\
                     3: \"a\"\n\
                     4: \"b\""
            .parse::<Rules>()
            .unwrap();
        assert!(rules.matches(0, "aab"));
        assert!(rules.matches(0, "aaab"));
        assert!(rules.matches(0, "aaaabb"));
        assert!(!rules.matches(0, "aabb"));
    }
}
