fn solve(_xs: &[usize]) -> usize {
    0
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p22.txt");
    let seq = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(&seq);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(&[]), 0);
    }
}
