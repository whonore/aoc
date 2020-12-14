use std::collections::HashMap;
use std::ops::BitAnd;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Mask(Vec<Option<bool>>);

impl FromStr for Mask {
    type Err = String;

    fn from_str(mask: &str) -> Result<Self, Self::Err> {
        let mask = mask
            .chars()
            .map(|c| match c {
                'X' => Ok(None),
                '0' => Ok(Some(false)),
                '1' => Ok(Some(true)),
                _ => Err(format!("Invalid mask char {}", c)),
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(mask))
    }
}

impl BitAnd<u64> for Mask {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        let mut out = rhs;
        for (bit, mask) in self.0.iter().rev().enumerate() {
            match mask {
                Some(true) => out |= 1 << bit,
                Some(false) => out &= !(1 << bit),
                _ => {}
            }
        }
        out
    }
}

type Seq = (Mask, Vec<(u64, u64)>);

fn parse_seq(seq: &str) -> Result<Seq, String> {
    let mut lines = seq.lines();
    let mask = lines.next().unwrap().parse::<Mask>()?;
    let mems = lines
        .map(|line| {
            let line = line.split(" = ").collect::<Vec<_>>();
            let mem = line[0]
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let val = line[1].parse::<u64>().unwrap();
            (mem, val)
        })
        .collect::<Vec<_>>();
    Ok((mask, mems))
}

fn solve(seqs: &[Seq]) -> u64 {
    let mem = seqs
        .iter()
        .flat_map(|(mask, mems)| mems.iter().map(move |mem| (mem.0, mask.clone() & mem.1)))
        .collect::<HashMap<u64, u64>>();
    mem.values().sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p14.txt");
    let seqs = input.split("mask = ").collect::<Vec<_>>();
    let seqs = seqs[1..]
        .iter()
        .map(|seq| parse_seq(seq))
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&seqs);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
        let mems = vec![(8, 11), (7, 101), (8, 0)];
        assert_eq!(solve(&[(mask, mems)]), 165);
    }
}
