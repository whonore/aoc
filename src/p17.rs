use std::collections::HashSet;
use std::iter;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }

    fn neighbors(&self) -> Vec<Point> {
        (-1i64..=1)
            .flat_map(|x| iter::repeat(x).zip(-1i64..=1))
            .flat_map(|x| iter::repeat(x).zip(-1i64..=1))
            .filter(|((x, y), z)| *x != 0 || *y != 0 || *z != 0)
            .map(|((x, y), z)| Point::new(self.x + x, self.y + y, self.z + z))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Cube {
    points: HashSet<Point>,
}

impl FromStr for Cube {
    type Err = String;

    fn from_str(plane: &str) -> Result<Self, Self::Err> {
        let points = plane
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Point::new(x as i64, y as i64, 0)),
                    '.' => None,
                    _ => panic!("Invalid char {}", c),
                })
            })
            .collect::<HashSet<_>>();
        Ok(Self { points })
    }
}

impl Cube {
    fn active(&self) -> usize {
        self.points.len()
    }

    fn active_around(&self, p: &Point) -> usize {
        p.neighbors()
            .iter()
            .filter(|n| self.points.contains(n))
            .count()
    }

    fn boot(self) -> Cube {
        CubeCycle(self).nth(5).unwrap()
    }
}

struct CubeCycle(Cube);

impl Iterator for CubeCycle {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        let keep = self
            .0
            .points
            .iter()
            .filter(|p| self.0.active_around(p) == 2 || self.0.active_around(p) == 3);
        let inactive = self
            .0
            .points
            .iter()
            .flat_map(|p| p.neighbors())
            .filter(|p| !self.0.points.contains(p));
        let add = inactive.filter(|p| self.0.active_around(p) == 3);
        self.0.points = keep.copied().chain(add).collect();
        Some(self.0.clone())
    }
}

fn solve(cube: Cube) -> usize {
    cube.boot().active()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p17.txt");
    let cube = input.parse::<Cube>()?;
    let out1 = solve(cube);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let cube = ".#.\n\
                    ..#\n\
                    ###"
        .parse::<Cube>()
        .unwrap();
        assert_eq!(solve(cube), 112);
    }
}
