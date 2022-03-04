use std::convert::{TryFrom, TryInto};

use itertools::Itertools;
use nom::{
    bits::complete::take,
    combinator::{map, map_res},
    multi::{count, many1},
    sequence::tuple,
    IResult,
};

type Bits<'a> = (&'a [u8], usize);

fn take_bool(bits: Bits<'_>) -> IResult<Bits<'_>, bool> {
    map(take(1_usize), |b: u8| b != 0)(bits)
}

fn split_bits(input: Bits<'_>, n: usize) -> (Bits<'_>, Bits<'_>) {
    if n == 0 {
        (input, (&[][..], 0))
    } else {
        let (bits, off) = input;
        let overflow = (n + off) % 8;
        let full_bytes = (n + off) / 8;
        let partial_bytes = full_bytes + if overflow != 0 { 1 } else { 0 };
        let out = &bits[..partial_bytes];
        ((&bits[full_bytes..], overflow), (out, off))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Lesser,
    Equal,
}

impl TryFrom<u8> for Operation {
    type Error = u8;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            5 => Ok(Self::Greater),
            6 => Ok(Self::Lesser),
            7 => Ok(Self::Equal),
            _ => Err(kind),
        }
    }
}

impl Operation {
    fn eval(self, mut args: impl Iterator<Item = u64>) -> u64 {
        match self {
            Self::Sum => args.sum(),
            Self::Product => args.product(),
            Self::Minimum => args.min().unwrap(),
            Self::Maximum => args.max().unwrap(),
            Self::Greater => u64::from(args.next().unwrap() > args.next().unwrap()),
            Self::Lesser => u64::from(args.next().unwrap() < args.next().unwrap()),
            Self::Equal => u64::from(args.next().unwrap() == args.next().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PacketKind {
    Literal,
    Operator(Operation),
}

impl TryFrom<u8> for PacketKind {
    type Error = u8;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        Ok(match kind {
            4 => Self::Literal,
            _ => Self::Operator(kind.try_into()?),
        })
    }
}

impl PacketKind {
    fn parse(bits: Bits<'_>) -> IResult<Bits<'_>, Self> {
        map_res(take(3_usize), u8::try_into)(bits)
    }
}

#[derive(Debug, Clone, Copy)]
enum LengthKind {
    Bits(usize),
    Count(usize),
}

impl LengthKind {
    fn parse(bits: Bits<'_>) -> IResult<Bits<'_>, Self> {
        let (bits, count) = take_bool(bits)?;
        if count {
            map(take(11_usize), Self::Count)(bits)
        } else {
            map(take(15_usize), Self::Bits)(bits)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Payload {
    Literal(u64),
    Operator(Vec<Packet>, Operation),
}

impl Payload {
    fn parse(bits: Bits<'_>) -> IResult<Bits<'_>, Self> {
        let (bits, kind) = PacketKind::parse(bits)?;
        match kind {
            PacketKind::Literal => map(Self::parse_literal, Self::Literal)(bits),
            PacketKind::Operator(op) => {
                map(Self::parse_operator, |pkts| Self::Operator(pkts, op))(bits)
            }
        }
    }

    fn parse_literal(mut bits: Bits<'_>) -> IResult<Bits<'_>, u64> {
        let mut sum: u64 = 0;
        loop {
            let (rest, more) = take_bool(bits)?;
            let (rest, v): (_, u64) = take(4_usize)(rest)?;
            bits = rest;
            sum = (sum << 4) + v;
            if !more {
                return Ok((bits, sum));
            }
        }
    }

    fn parse_operator(bits: Bits<'_>) -> IResult<Bits<'_>, Vec<Packet>> {
        let (bits, kind) = LengthKind::parse(bits)?;
        match kind {
            LengthKind::Bits(n) => {
                let (rest, bits) = split_bits(bits, n);
                let (_, packets) = many1(Packet::parse_bits)(bits)?;
                Ok((rest, packets))
            }
            LengthKind::Count(n) => count(Packet::parse_bits, n)(bits),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Self::Literal(n) => *n,
            Self::Operator(pkts, op) => op.eval(pkts.iter().map(Packet::eval)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    payload: Payload,
}

fn hex_to_bits(hex: &str) -> Vec<u8> {
    hex.chars()
        .chunks(2)
        .into_iter()
        .map(|hex| u8::from_str_radix(&hex.collect::<String>(), 16).unwrap())
        .collect()
}

impl Packet {
    fn parse(hex: &str) -> Result<Self, String> {
        let bits = hex_to_bits(hex);
        Self::parse_bits((&bits, 0))
            .map(|(_, packet)| packet)
            .map_err(|e| format!("{:?}", e))
    }

    fn parse_bits(bits: Bits<'_>) -> IResult<Bits<'_>, Self> {
        map(
            tuple((Self::parse_version, Payload::parse)),
            |(version, payload)| Self { version, payload },
        )(bits)
    }

    fn parse_version(bits: Bits<'_>) -> IResult<Bits<'_>, u8> {
        take(3_usize)(bits)
    }

    fn version_sum(&self) -> u64 {
        u64::from(self.version)
            + match &self.payload {
                Payload::Literal(_) => 0,
                Payload::Operator(pkts, _) => pkts.iter().map(Self::version_sum).sum(),
            }
    }

    fn eval(&self) -> u64 {
        self.payload.eval()
    }
}

fn part1(hex: &str) -> Result<u64, String> {
    Packet::parse(hex).map(|pkt| pkt.version_sum())
}

fn part2(hex: &str) -> Result<u64, String> {
    Packet::parse(hex).map(|pkt| pkt.eval())
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d16.txt");
    let hex = input.trim();
    let out1 = part1(hex)?;
    let out2 = part2(hex)?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_bits() {
        let bits = [0b1001_0110, 0b0011_1100, 0b0100_0010, 0b1000_0001].as_ref();
        assert_eq!(split_bits((bits, 0), 0), ((bits, 0), (&[][..], 0)));
        assert_eq!(
            split_bits((bits, 0), 1),
            ((bits, 1), (&[0b1001_0110][..], 0))
        );
        assert_eq!(
            split_bits((bits, 1), 3),
            ((bits, 4), (&[0b1001_0110][..], 1))
        );
        assert_eq!(
            split_bits((bits, 0), 11),
            ((&bits[1..], 3), (&[0b1001_0110, 0b0011_1100][..], 0))
        );
        assert_eq!(
            split_bits((bits, 4), 11),
            ((&bits[1..], 7), (&[0b1001_0110, 0b0011_1100][..], 4))
        );
        assert_eq!(
            split_bits((bits, 4), 4),
            ((&bits[1..], 0), (&[0b1001_0110][..], 4))
        );
    }

    #[test]
    fn text_hex_to_bits() {
        assert_eq!(hex_to_bits("00"), vec![0b0000_0000]);
        assert_eq!(hex_to_bits("FE"), vec![0b1111_1110]);
        assert_eq!(
            hex_to_bits("D2FE28"),
            vec![0b1101_0010, 0b1111_1110, 0b0010_1000]
        );
    }

    #[test]
    fn test_parse_literal() {
        assert_eq!(
            Packet::parse("D2FE28"),
            Ok(Packet {
                version: 6,
                payload: Payload::Literal(2021)
            })
        );
    }

    #[test]
    fn test_parse_operator_bits() {
        assert_eq!(
            Packet::parse("38006F45291200"),
            Ok(Packet {
                version: 1,
                payload: Payload::Operator(
                    vec![
                        Packet {
                            version: 6,
                            payload: Payload::Literal(10)
                        },
                        Packet {
                            version: 2,
                            payload: Payload::Literal(20)
                        },
                    ],
                    Operation::Lesser
                )
            })
        );
    }

    #[test]
    fn test_parse_operator_count() {
        assert_eq!(
            Packet::parse("EE00D40C823060"),
            Ok(Packet {
                version: 7,
                payload: Payload::Operator(
                    vec![
                        Packet {
                            version: 2,
                            payload: Payload::Literal(1)
                        },
                        Packet {
                            version: 4,
                            payload: Payload::Literal(2)
                        },
                        Packet {
                            version: 1,
                            payload: Payload::Literal(3)
                        },
                    ],
                    Operation::Maximum
                )
            })
        );
    }

    #[test]
    fn test01() {
        assert_eq!(part1("8A004A801A8002F478"), Ok(16));
        assert_eq!(part1("620080001611562C8802118E34"), Ok(12));
        assert_eq!(part1("C0015000016115A2E0802F182340"), Ok(23));
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), Ok(31));
    }

    #[test]
    fn test02() {
        assert_eq!(part2("C200B40A82"), Ok(3));
        assert_eq!(part2("04005AC33890"), Ok(54));
        assert_eq!(part2("880086C3E88112"), Ok(7));
        assert_eq!(part2("CE00C43D881120"), Ok(9));
        assert_eq!(part2("D8005AC2A8F0"), Ok(1));
        assert_eq!(part2("F600BC2D8F"), Ok(0));
        assert_eq!(part2("9C005AC2F8F0"), Ok(0));
        assert_eq!(part2("9C0141080250320F1802104A08"), Ok(1));
    }
}
