use std::ops::Add;

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64 as parse_u64,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum LeftRight<A> {
    Left(A),
    Right(A),
}
use LeftRight::*;

impl<A> LeftRight<A> {
    #[allow(clippy::missing_const_for_fn)]
    fn get(self) -> A {
        match self {
            Left(x) | Right(x) => x,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Num {
    Normal(u64),
    Pair(Box<Self>, Box<Self>),
}

impl Add for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(self), Box::new(rhs)).reduce()
    }
}

impl Num {
    fn parse(num: &str) -> Result<Self, String> {
        Self::parse_pair(num)
            .map(|(_, num)| num)
            .map_err(|e| format!("{:?}", e))
    }

    fn parse_pair(num: &str) -> IResult<&str, Self> {
        let parse_left = alt((Self::parse_normal, Self::parse_pair));
        let parse_right = alt((Self::parse_normal, Self::parse_pair));
        map(
            delimited(
                tag("["),
                separated_pair(parse_left, tag(","), parse_right),
                tag("]"),
            ),
            |(x, y)| Self::Pair(Box::new(x), Box::new(y)),
        )(num)
    }

    fn parse_normal(num: &str) -> IResult<&str, Self> {
        map(parse_u64, Self::Normal)(num)
    }

    fn reduce(mut self) -> Self {
        loop {
            let (num, exploded) = self.explode();
            self = num;
            if exploded {
                continue;
            }
            let (num, split) = self.split();
            self = num;
            if split {
                continue;
            }
            return self;
        }
    }

    fn explode(self) -> (Self, bool) {
        let (num, _, exploded) = self.explode_rec(0);
        (num, exploded)
    }

    fn explode_rec(self, depth: u64) -> (Self, Option<LeftRight<u64>>, bool) {
        match self {
            Self::Normal(_) => (self, None, false),
            Self::Pair(left, right) => {
                if depth == 3 && matches!(*left, Self::Pair(_, _)) {
                    let lval = left.left().unwrap().value().unwrap();
                    let rval = left.right().unwrap().value().unwrap();
                    let right = right.add_to_first(Right(rval));
                    let num = Self::Pair(Box::new(Self::Normal(0)), Box::new(right));
                    (num, Some(Left(lval)), true)
                } else if depth == 3 && matches!(*right, Self::Pair(_, _)) {
                    let lval = right.left().unwrap().value().unwrap();
                    let rval = right.right().unwrap().value().unwrap();
                    let left = left.add_to_first(Left(lval));
                    let num = Self::Pair(Box::new(left), Box::new(Self::Normal(0)));
                    (num, Some(Right(rval)), true)
                } else {
                    let (left, val, lexploded) = left.explode_rec(depth + 1);
                    if lexploded {
                        let (right, val) = if let Some(Right(val)) = val {
                            (Box::new(right.add_to_first(Right(val))), None)
                        } else {
                            (right, val)
                        };
                        (Self::Pair(Box::new(left), right), val, true)
                    } else {
                        let (right, val, rexploded) = right.explode_rec(depth + 1);
                        let (left, val) = match val {
                            Some(Left(val)) if rexploded => (left.add_to_first(Left(val)), None),
                            _ => (left, val),
                        };
                        (Self::Pair(Box::new(left), Box::new(right)), val, rexploded)
                    }
                }
            }
        }
    }

    fn add_to_first(self, val: LeftRight<u64>) -> Self {
        match self {
            Self::Normal(n) => Self::Normal(n + val.get()),
            Self::Pair(left, right) => match val {
                Left(_) => Self::Pair(left, Box::new(right.add_to_first(val))),
                Right(_) => Self::Pair(Box::new(left.add_to_first(val)), right),
            },
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Self::Normal(n) if n >= 10 => {
                let left = n / 2;
                let right = left + if n % 2 == 0 { 0 } else { 1 };
                (
                    Self::Pair(Box::new(Self::Normal(left)), Box::new(Self::Normal(right))),
                    true,
                )
            }
            Self::Normal(_) => (self, false),
            Self::Pair(left, right) => {
                let (left, lsplit) = left.split();
                if lsplit {
                    (Self::Pair(Box::new(left), right), true)
                } else {
                    let (right, rsplit) = right.split();
                    (Self::Pair(Box::new(left), Box::new(right)), rsplit)
                }
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Normal(n) => *n,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    const fn value(&self) -> Option<u64> {
        match self {
            Self::Normal(n) => Some(*n),
            Self::Pair(_, _) => None,
        }
    }

    const fn left(&self) -> Option<&Self> {
        match self {
            Self::Pair(left, _) => Some(left),
            Self::Normal(_) => None,
        }
    }

    const fn right(&self) -> Option<&Self> {
        match self {
            Self::Pair(_, right) => Some(right),
            Self::Normal(_) => None,
        }
    }
}

fn part1(nums: Vec<Num>) -> u64 {
    nums.into_iter().reduce(|x, y| x + y).unwrap().magnitude()
}

fn part2(nums: Vec<Num>) -> u64 {
    nums.into_iter()
        .permutations(2)
        .map(|nums| (nums[0].clone() + nums[1].clone()).magnitude())
        .max()
        .unwrap()
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d18.txt");
    let nums = input
        .lines()
        .map(Num::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(nums.clone());
    let out2 = part2(nums);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let n1 = Num::Pair(Box::new(Num::Normal(1)), Box::new(Num::Normal(2)));
        let n2 = Num::Pair(Box::new(n1.clone()), Box::new(Num::Normal(3)));
        let n3 = Num::Pair(Box::new(Num::Normal(3)), Box::new(n1.clone()));
        let n4 = Num::Pair(Box::new(n1.clone()), Box::new(n1.clone()));
        assert_eq!(Num::parse("[1,2]"), Ok(n1));
        assert_eq!(Num::parse("[[1,2],3]"), Ok(n2));
        assert_eq!(Num::parse("[3,[1,2]]"), Ok(n3));
        assert_eq!(Num::parse("[[1,2],[1,2]]"), Ok(n4));
    }

    #[test]
    fn test_explode() {
        let test = |x, y| {
            assert_eq!(
                Num::parse(x).unwrap().explode(),
                (Num::parse(y).unwrap(), true)
            );
        };
        test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        let test = |x, y| assert_eq!(Num::Normal(x).split(), (Num::parse(y).unwrap(), true));
        test(10, "[5,5]");
        test(11, "[5,6]");
        test(12, "[6,6]");
    }

    #[test]
    fn test_add() {
        let test = |xs: &[&str], y| {
            assert_eq!(
                xs.iter()
                    .copied()
                    .map(Num::parse)
                    .map(Result::unwrap)
                    .reduce(|x, y| x + y)
                    .unwrap(),
                Num::parse(y).unwrap()
            );
        };
        test(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        test(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        test(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
        test(
            &[
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ],
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    fn test_magnitude() {
        let test = |x, y| assert_eq!(Num::parse(x).unwrap().magnitude(), y);
        test("[9,1]", 29);
        test("[1,9]", 21);
        test("[[9,1],[1,9]]", 129);
        test("[[1,2],[[3,4],5]]", 143);
        test("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        test("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        test("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        test("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        test(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }

    #[test]
    fn test01() {
        let xs = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .copied()
        .map(Num::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        assert_eq!(part1(xs), 4140);
    }

    #[test]
    fn test02() {
        let xs = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .copied()
        .map(Num::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        assert_eq!(part2(xs), 3993);
    }
}
