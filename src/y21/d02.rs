use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Forward(u64),
    Down(u64),
    Up(u64),
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
    horz: u64,
    depth: u64,
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


fn part1(dirs: &[Dir]) -> u64 {
    let pos = dirs.iter().fold(Pos::new(), |pos, dir| pos.step(*dir));
    pos.horz * pos.depth
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
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
}
