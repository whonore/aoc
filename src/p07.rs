use std::collections::hash_map::{HashMap, Keys};
use std::str::FromStr;

struct Rules {
    rules: HashMap<String, Option<Vec<(String, usize)>>>,
}

impl FromStr for Rules {
    type Err = String;

    fn from_str(rules: &str) -> Result<Self, Self::Err> {
        let rules = rules
            .lines()
            .map(|rule| {
                let words = rule.split_whitespace().collect::<Vec<_>>();
                let outer = words[..2].join(" ");
                let inner = words[4..].join(" ");
                let inner = if inner.starts_with("no") {
                    None
                } else {
                    Some(
                        inner
                            .split(", ")
                            .map(|rule| rule.split_whitespace().collect::<Vec<_>>())
                            .map(|rule| {
                                let n = rule[0].parse::<usize>().unwrap();
                                let bag = rule[1..3].join(" ");
                                (bag, n)
                            })
                            .collect::<Vec<_>>(),
                    )
                };
                Ok((outer, inner))
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Rules { rules })
    }
}

impl Rules {
    fn bags(&self) -> Keys<String, Option<Vec<(String, usize)>>> {
        self.rules.keys()
    }

    fn can_hold(&self, outer: &str, inner: &str) -> bool {
        if let Some(bags) = &self.rules[outer] {
            bags.iter()
                .any(|(bag, _)| bag == inner || self.can_hold(bag, inner))
        } else {
            false
        }
    }
}

fn solve(rules: &Rules, bag: &str) -> usize {
    rules
        .bags()
        .filter(|outer| rules.can_hold(outer, bag))
        .count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p07.txt");
    let rules = input.parse::<Rules>()?;
    let out1 = solve(&rules, "shiny gold");
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags."
            .parse::<Rules>();
        assert!(rules.is_ok());
    }

    #[test]
    fn test01() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags."
            .parse::<Rules>()
            .unwrap();
        assert_eq!(solve(&rules, "shiny gold"), 4);
    }
}
