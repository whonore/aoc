fn take3<A>(it: &mut impl Iterator<Item = A>) -> [A; 3] {
    [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
}

fn part1(p1: u64, p2: u64) -> u64 {
    let mut dice = (1..=100).cycle();
    let mut pos = [p1 - 1, p2 - 1];
    let mut scores = [0, 0];
    let mut player = 0;
    let mut rolls = 0;
    while scores.iter().all(|&score| score < 1000) {
        pos[player] = (pos[player] + take3(&mut dice).iter().sum::<u64>()) % 10;
        scores[player] += pos[player] + 1;
        player = (player + 1) % 2;
        rolls += 3;
    }
    scores.iter().min().unwrap() * rolls
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d21.txt");
    let (p1, p2) = {
        let parse = |line: &str| u64::from(line.chars().last().unwrap().to_digit(10).unwrap());
        let mut lines = input.lines();
        (parse(lines.next().unwrap()), parse(lines.next().unwrap()))
    };
    let out1 = part1(p1, p2);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(part1(4, 8), 739785);
    }
}
