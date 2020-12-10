use std::collections::HashMap;

fn distribution(jolts: &[u32]) -> usize {
    let min = jolts[0];
    let diffs = jolts.windows(2).map(|js| js[1] - js[0]);
    let (ones, rest): (Vec<_>, _) = diffs.partition(|d| *d == 1);
    (if min == 1 { 1 } else { 0 } + ones.iter().count())
        * (if min == 3 { 1 } else { 0 } + 1 + rest.iter().filter(|d| **d == 3).count())
}

fn arrangements(jolts: &[u32]) -> u64 {
    let mut memo = HashMap::<u32, u64>::new();
    for jolt in jolts.iter().rev() {
        let mut count = 0;
        for off in 1..=3 {
            if jolts.contains(&(jolt + off)) {
                count += memo[&(jolt + off)];
            }
        }
        memo.insert(*jolt, if count != 0 { count } else { 1 });
    }
    jolts.iter().take_while(|j| **j <= 3).map(|j| memo[j]).sum()
}

fn solve(jolts: &mut [u32]) -> (usize, u64) {
    jolts.sort();
    (distribution(&jolts), arrangements(&jolts))
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p10.txt");
    let mut jolts = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let (out1, out2) = solve(&mut jolts);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut jolts = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(solve(&mut jolts), (7 * 5, 8));
    }

    #[test]
    fn test02() {
        let mut jolts = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(solve(&mut jolts), (22 * 10, 19208));
    }
}
