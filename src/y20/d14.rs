use std::collections::HashMap;
use std::fmt;
use std::iter;
use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
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

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|bit| match bit {
                    Some(true) => "1",
                    Some(false) => "0",
                    None => "X",
                })
                .collect::<String>()
        )
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
                None => {}
            }
        }
        out
    }
}

fn bit_at(n: u64, bit: u8) -> bool {
    match (n >> bit) & 1 {
        0 => false,
        1 => true,
        _ => panic!("Impossible bit_at"),
    }
}

impl BitOr<u64> for Mask {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation)]
    fn bitor(self, rhs: u64) -> Self::Output {
        let mut out = vec![];
        for (bit, mask) in self.0.iter().rev().enumerate() {
            out.push(match mask {
                Some(true) => Some(true),
                Some(false) => Some(bit_at(rhs, bit as u8)),
                None => None,
            });
        }
        out.reverse();
        Self(out)
    }
}

impl Mask {
    fn expand(&self) -> Vec<u64> {
        let mut vals = vec![0];
        for bit in &self.0 {
            match bit {
                Some(true) => {
                    for v in &mut vals {
                        *v = (*v << 1) | 1;
                    }
                }
                Some(false) => {
                    for v in &mut vals {
                        *v <<= 1;
                    }
                }
                None => {
                    let mut vals2 = vals.clone();
                    for v in &mut vals {
                        *v = (*v << 1) | 1;
                    }
                    for v in &mut vals2 {
                        *v <<= 1;
                    }
                    vals.append(&mut vals2);
                }
            }
        }
        vals
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

#[derive(Copy, Clone)]
enum Mode {
    V1,
    V2,
}
use Mode::*;

fn solve(seqs: &[Seq], mode: Mode) -> u64 {
    let mem = match mode {
        V1 => seqs
            .iter()
            .flat_map(|(mask, mems)| mems.iter().map(move |mem| (mem.0, mask.clone() & mem.1)))
            .collect::<HashMap<u64, u64>>(),
        V2 => seqs
            .iter()
            .flat_map(|(mask, mems)| {
                mems.iter().flat_map(move |mem| {
                    (mask.clone() | mem.0)
                        .expand()
                        .iter()
                        .copied()
                        .zip(iter::repeat(mem.1))
                        .collect::<Vec<_>>()
                })
            })
            .collect::<HashMap<u64, u64>>(),
    };
    mem.values().sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d14.txt");
    let seqs = input.split("mask = ").collect::<Vec<_>>();
    let seqs = seqs[1..]
        .iter()
        .map(|seq| parse_seq(seq))
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&seqs, V1);
    let out2 = solve(&seqs, V2);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let mask: Mask = "01XX".parse().unwrap();
        assert_eq!(mask.expand(), vec![0b0111, 0b0101, 0b0110, 0b0100]);
    }

    #[test]
    fn test01() {
        let mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
        let mems = vec![(8, 11), (7, 101), (8, 0)];
        assert_eq!(solve(&[(mask, mems)], V1), 165);
    }

    #[test]
    fn test02() {
        let mask1: Mask = "000000000000000000000000000000X1001X".parse().unwrap();
        let mems1 = vec![(42, 100)];
        let mask2: Mask = "00000000000000000000000000000000X0XX".parse().unwrap();
        let mems2 = vec![(26, 1)];
        assert_eq!(solve(&[(mask1, mems1), (mask2, mems2)], V2), 208);
    }
}
