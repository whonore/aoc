use std::collections::HashSet;

fn solve(qs: &[&str]) -> usize {
    let qs = qs
        .iter()
        .map(|q| q.chars().filter(|c| *c != '\n').collect::<HashSet<_>>());
    qs.map(|q| q.len()).sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p06.txt");
    let qs: Vec<_> = input.split("\n\n").collect();
    let out1 = solve(&qs);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            solve(&[
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
            ]),
            11
        )
    }
}
