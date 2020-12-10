fn solve(jolts: &mut [u32]) -> usize {
    jolts.sort();
    let min = jolts[0];
    let diffs = jolts.windows(2).map(|js| js[1] - js[0]);
    let (ones, rest): (Vec<_>, _) = diffs.partition(|d| *d == 1);
    (if min == 1 { 1 } else { 0 } + ones.iter().count())
        * (if min == 3 { 1 } else { 0 } + 1 + rest.iter().filter(|d| **d == 3).count())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p10.txt");
    let mut jolts = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(&mut jolts);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut jolts = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(solve(&mut jolts), 7 * 5);
        let mut jolts = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(solve(&mut jolts), 22 * 10);
    }
}
