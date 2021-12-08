use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Forward(i64),
    Down(i64),
    Up(i64),
}
use Dir::*;

impl FromStr for Dir {
    type Err = String;

    fn from_str(dir: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = match dir.split(' ').collect::<Vec<_>>()[..] {
            [dir, amt] => Ok((dir, amt)),
            _ => Err("Too few fields"),
        }?;
        let amt = amt.parse().map_err(|_| "Invalid amount")?;
        match dir {
            "forward" => Ok(Forward(amt)),
            "down" => Ok(Down(amt)),
            "up" => Ok(Up(amt)),
            _ => Err("Invalid direction".into()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    horz: i64,
    depth: i64,
}

impl Pos {
    const fn new() -> Self {
        Self { horz: 0, depth: 0 }
    }

    const fn step(&self, dir: Dir) -> Self {
        match dir {
            Forward(x) => Self {
                horz: self.horz + x,
                ..*self
            },
            Down(x) => Self {
                depth: self.depth + x,
                ..*self
            },
            Up(x) => Self {
                depth: self.depth - x,
                ..*self
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PosAim {
    horz: i64,
    depth: i64,
    aim: i64,
}

impl PosAim {
    const fn new() -> Self {
        Self {
            horz: 0,
            depth: 0,
            aim: 0,
        }
    }

    const fn step(&self, dir: Dir) -> Self {
        match dir {
            Forward(x) => Self {
                horz: self.horz + x,
                depth: self.depth + (x * self.aim),
                ..*self
            },
            Down(x) => Self {
                aim: self.aim + x,
                ..*self
            },
            Up(x) => Self {
                aim: self.aim - x,
                ..*self
            },
        }
    }
}

fn part1(dirs: &[Dir]) -> i64 {
    let pos = dirs.iter().fold(Pos::new(), |pos, dir| pos.step(*dir));
    pos.horz * pos.depth
}

fn part2(dirs: &[Dir]) -> i64 {
    let pos = dirs.iter().fold(PosAim::new(), |pos, dir| pos.step(*dir));
    pos.horz * pos.depth
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d02.txt");
    let dirs = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&dirs);
    let out2 = part2(&dirs);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let dirs = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(part1(&dirs), 150);
    }

    #[test]
    fn test02() {
        let dirs = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(part2(&dirs), 900);
    }
}
