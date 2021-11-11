use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Mode {
    Any,
    All,
}
use Mode::*;

fn solve(qs: &[&str], mode: Mode) -> usize {
    qs.iter()
        .map(|q| {
            let mut qs = q
                .lines()
                .map(|person| person.chars().collect::<HashSet<_>>());
            let first = qs.next().unwrap();
            match mode {
                Any => qs.fold(first, |acc, q| acc.union(&q).copied().collect()),
                All => qs.fold(first, |acc, q| acc.intersection(&q).copied().collect()),
            }
            .len()
        })
        .sum()
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d06.txt");
    let qs: Vec<_> = input.split("\n\n").collect();
    let out1 = solve(&qs, Any);
    let out2 = solve(&qs, All);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            solve(
                &[
                    "abc",
                    "a\n\
                     b\n\
                     c",
                    "ab\n\
                     ac",
                    "a\n\
                     a\n\
                     a\n\
                     a",
                    "b"
                ],
                Any
            ),
            11
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            solve(
                &[
                    "abc",
                    "a\n\
                     b\n\
                     c",
                    "ab\n\
                     ac",
                    "a\n\
                     a\n\
                     a\n\
                     a",
                    "b"
                ],
                All
            ),
            6
        );
    }
}
