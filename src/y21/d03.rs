use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Bin(Vec<u64>);

impl FromStr for Bin {
    type Err = String;

    fn from_str(bin: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bin.chars().map(|c| c.to_digit(2).unwrap().into()).collect(),
        ))
    }
}

impl Add for Bin {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| x + y)
                .collect(),
        )
    }
}

impl Bin {
    fn to_u64(&self) -> u64 {
        #[allow(clippy::cast_possible_truncation)]
        let s: String = self
            .0
            .iter()
            .map(|c| char::from_digit(*c as u32, 2).unwrap())
            .collect();
        u64::from_str_radix(&s, 2).unwrap()
    }
}

fn part1(bins: &[Bin]) -> u64 {
    let n = bins.len() as u64;
    let len = bins[0].0.len() as u64;
    let mask = (1 << len) - 1;
    let cts: Bin = bins.iter().cloned().reduce(|x, y| x + y).unwrap();
    let gamma = Bin(cts
        .0
        .iter()
        .map(|x| if *x > n / 2 { 1 } else { 0 })
        .collect())
    .to_u64();
    let epsilon = !gamma & mask;
    gamma * epsilon
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d03.txt");
    let bins = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&bins);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let bins = [
            Bin(vec![0, 0, 1, 0, 0]),
            Bin(vec![1, 1, 1, 1, 0]),
            Bin(vec![1, 0, 1, 1, 0]),
            Bin(vec![1, 0, 1, 1, 1]),
            Bin(vec![1, 0, 1, 0, 1]),
            Bin(vec![0, 1, 1, 1, 1]),
            Bin(vec![0, 0, 1, 1, 1]),
            Bin(vec![1, 1, 1, 0, 0]),
            Bin(vec![1, 0, 0, 0, 0]),
            Bin(vec![1, 1, 0, 0, 1]),
            Bin(vec![0, 0, 0, 1, 0]),
            Bin(vec![0, 1, 0, 1, 0]),
        ];
        assert_eq!(part1(&bins), 198);
    }
}
