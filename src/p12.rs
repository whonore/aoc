use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}
use Dir::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Rotate {
    Left,
    Right,
}
use Rotate::*;

impl Add<Rotate> for Dir {
    type Output = Self;

    fn add(self, rot: Rotate) -> Self {
        match (self, rot) {
            (North, Left) => West,
            (North, Right) => East,
            (East, Left) => North,
            (East, Right) => South,
            (South, Left) => East,
            (South, Right) => West,
            (West, Left) => South,
            (West, Right) => North,
        }
    }
}

impl AddAssign<Rotate> for Dir {
    fn add_assign(&mut self, rot: Rotate) {
        *self = *self + rot;
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Action {
    Go(Option<Dir>, u32),
    Turn(Rotate, u32),
}
use Action::*;

impl FromStr for Action {
    type Err = String;

    fn from_str(act: &str) -> Result<Self, Self::Err> {
        let (act, amt) = act.split_at(1);
        let amt = amt
            .parse::<u32>()
            .map_err(|_| format!("Invalid action amount {}", amt))?;
        match act {
            "N" => Ok(Go(Some(North), amt)),
            "S" => Ok(Go(Some(South), amt)),
            "E" => Ok(Go(Some(East), amt)),
            "W" => Ok(Go(Some(West), amt)),
            "F" => Ok(Go(None, amt)),
            "L" if amt % 90 == 0 => Ok(Turn(Left, amt / 90)),
            "R" if amt % 90 == 0 => Ok(Turn(Right, amt / 90)),
            _ => Err(format!("Invalid action {} {}", act, amt)),
        }
    }
}

struct Pos {
    x: i32,
    y: i32,
    dir: Dir,
}

impl Pos {
    fn new() -> Self {
        Pos {
            x: 0,
            y: 0,
            dir: East,
        }
    }

    fn abs(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn go(&mut self, act: &Action) {
        match act {
            Go(dir, amt) => {
                let dir = dir.unwrap_or(self.dir);
                match dir {
                    North => self.y += *amt as i32,
                    South => self.y -= *amt as i32,
                    East => self.x += *amt as i32,
                    West => self.x -= *amt as i32,
                }
            }
            Turn(rot, amt) => {
                for _ in 0..*amt {
                    self.dir += *rot;
                }
            }
        }
    }
}

fn solve(acts: &[Action]) -> u32 {
    let mut pos = Pos::new();
    for act in acts {
        pos.go(act);
    }
    pos.abs()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p12.txt");
    let acts = input
        .lines()
        .map(|act| act.parse::<Action>())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&acts);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let acts = [
            Go(None, 10),
            Go(Some(North), 3),
            Go(None, 7),
            Turn(Right, 1),
            Go(None, 11),
        ];
        assert_eq!(solve(&acts), 25);
    }
}
