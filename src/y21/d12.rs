use std::collections::{HashMap, HashSet};

use multimap::MultiMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<'c> {
    Start,
    End,
    Big(&'c str),
    Small(&'c str),
}

impl<'c> Cave<'c> {
    fn new(cave: &'c str) -> Self {
        match cave {
            "start" => Self::Start,
            "end" => Self::End,
            _ if cave.chars().next().unwrap().is_uppercase() => Self::Big(cave),
            _ => Self::Small(cave),
        }
    }

    const fn is_big(&self) -> bool {
        matches!(self, Cave::Big(_))
    }

    const fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
}

#[derive(Debug, Clone)]
struct Caves<'c> {
    caves: MultiMap<Cave<'c>, Cave<'c>>,
}

impl<'c> Caves<'c> {
    fn new(caves: &'c str) -> Self {
        let mut map = MultiMap::new();
        caves
            .lines()
            .map(|line| {
                let mut caves = line.split('-');
                (
                    Cave::new(caves.next().unwrap()),
                    Cave::new(caves.next().unwrap()),
                )
            })
            .for_each(|(x, y)| {
                map.insert(x, y);
                map.insert(y, x);
            });
        Self { caves: map }
    }

    fn navigate(&self, start: Cave<'c>, visited: &mut HashSet<Cave<'c>>) -> u64 {
        if start == Cave::End {
            1
        } else {
            let mut paths = 0;
            for next in self
                .caves
                .get_vec(&start)
                .into_iter()
                .flatten()
                .filter(|next| !visited.contains(next) || next.is_big())
                .collect::<Vec<_>>()
            {
                visited.insert(*next);
                paths += self.navigate(*next, visited);
                visited.remove(next);
            }
            paths
        }
    }

    fn navigate_twice(
        &self,
        start: Cave<'c>,
        visited: &mut HashMap<Cave<'c>, u64>,
        twice: bool,
    ) -> u64 {
        if start == Cave::End {
            1
        } else {
            let mut paths = 0;
            for next in self
                .caves
                .get_vec(&start)
                .into_iter()
                .flatten()
                .filter(|next| {
                    visited.get(next).copied().unwrap_or(0) == 0
                        || next.is_big()
                        || (next.is_small() && !twice)
                })
                .collect::<Vec<_>>()
            {
                let twice =
                    twice || (visited.get(next).copied().unwrap_or(0) == 1 && next.is_small());
                visited.entry(*next).and_modify(|v| *v += 1).or_insert(1);
                paths += self.navigate_twice(*next, visited, twice);
                visited.entry(*next).and_modify(|v| *v -= 1);
            }
            paths
        }
    }
}

fn part1<'c>(caves: &Caves<'c>) -> u64 {
    let mut visited = HashSet::<Cave<'c>>::new();
    visited.insert(Cave::Start);
    caves.navigate(Cave::Start, &mut visited)
}

fn part2<'c>(caves: &Caves<'c>) -> u64 {
    let mut visited = HashMap::<Cave<'c>, u64>::new();
    visited.insert(Cave::Start, 1);
    caves.navigate_twice(Cave::Start, &mut visited, false)
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d12.txt");
    let caves = Caves::new(input);
    let out1 = part1(&caves);
    let out2 = part2(&caves);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let caves = Caves::new(
            "start-A\n\
             start-b\n\
             A-c\n\
             A-b\n\
             b-d\n\
             A-end\n\
             b-end",
        );
        assert_eq!(part1(&caves), 10);

        let caves = Caves::new(
            "dc-end\n\
             HN-start\n\
             start-kj\n\
             dc-start\n\
             dc-HN\n\
             LN-dc\n\
             HN-end\n\
             kj-sa\n\
             kj-HN\n\
             kj-dc",
        );
        assert_eq!(part1(&caves), 19);

        let caves = Caves::new(
            "fs-end\n\
             he-DX\n\
             fs-he\n\
             start-DX\n\
             pj-DX\n\
             end-zg\n\
             zg-sl\n\
             zg-pj\n\
             pj-he\n\
             RW-he\n\
             fs-DX\n\
             pj-RW\n\
             zg-RW\n\
             start-pj\n\
             he-WI\n\
             zg-he\n\
             pj-fs\n\
             start-RW",
        );
        assert_eq!(part1(&caves), 226);
    }

    #[test]
    fn test02() {
        let caves = Caves::new(
            "start-A\n\
             start-b\n\
             A-c\n\
             A-b\n\
             b-d\n\
             A-end\n\
             b-end",
        );
        assert_eq!(part2(&caves), 36);

        let caves = Caves::new(
            "dc-end\n\
             HN-start\n\
             start-kj\n\
             dc-start\n\
             dc-HN\n\
             LN-dc\n\
             HN-end\n\
             kj-sa\n\
             kj-HN\n\
             kj-dc",
        );
        assert_eq!(part2(&caves), 103);

        let caves = Caves::new(
            "fs-end\n\
             he-DX\n\
             fs-he\n\
             start-DX\n\
             pj-DX\n\
             end-zg\n\
             zg-sl\n\
             zg-pj\n\
             pj-he\n\
             RW-he\n\
             fs-DX\n\
             pj-RW\n\
             zg-RW\n\
             start-pj\n\
             he-WI\n\
             zg-he\n\
             pj-fs\n\
             start-RW",
        );
        assert_eq!(part2(&caves), 3509);
    }
}
