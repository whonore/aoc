use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::marker::PhantomData;
use std::ops::Add;
use std::str::FromStr;

trait Dimension: core::fmt::Debug + Clone + Eq + Hash {
    const DIMS: u8;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct D3;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct D4;

impl Dimension for D3 {
    const DIMS: u8 = 3;
}

impl Dimension for D4 {
    const DIMS: u8 = 4;
}

fn flat_zip<I, J>(xs: I, ys: J) -> Vec<Vec<J::Item>>
where
    I: IntoIterator<Item = Vec<J::Item>>,
    J: IntoIterator,
{
    xs.into_iter()
        .zip(ys)
        .map(|(mut xs, y)| {
            xs.push(y);
            xs
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point<D: Dimension> {
    ps: Vec<i64>,
    dim: PhantomData<D>,
}

impl<D: Dimension> Add for &'_ Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(
            &self
                .ps
                .iter()
                .zip(rhs.ps.iter())
                .map(|(x, y)| x + y)
                .collect::<Vec<_>>(),
        )
    }
}

impl<D: Dimension> Point<D> {
    fn new2(x: i64, y: i64) -> Self {
        Self {
            ps: [x, y]
                .iter()
                .copied()
                .chain(vec![0_i64; D::DIMS as usize - 2])
                .collect(),
            dim: PhantomData,
        }
    }

    fn new(ps: &[i64]) -> Self {
        assert!(ps.len() == D::DIMS as usize);
        Self {
            ps: ps.to_vec(),
            dim: PhantomData,
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut offs: Vec<Vec<i64>> = vec![vec![-1], vec![0], vec![1]];
        for _ in 1..D::DIMS {
            offs = offs
                .iter()
                .flat_map(|off| flat_zip(iter::repeat(off).cloned(), -1..=1))
                .collect();
        }
        offs.iter()
            .filter(|ps| ps.iter().any(|p| *p != 0))
            .map(|ps| self + &Self::new(ps))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Cube<D: Dimension> {
    points: HashSet<Point<D>>,
}

impl<D: Dimension> FromStr for Cube<D> {
    type Err = String;

    fn from_str(plane: &str) -> Result<Self, Self::Err> {
        let points = plane
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Point::new2(x as i64, y as i64)),
                    '.' => None,
                    _ => panic!("Invalid char {}", c),
                })
            })
            .collect::<HashSet<_>>();
        Ok(Self { points })
    }
}

impl<D: Dimension> Cube<D> {
    fn active(&self) -> usize {
        self.points.len()
    }

    fn active_around(&self, p: &Point<D>) -> usize {
        p.neighbors()
            .iter()
            .filter(|n| self.points.contains(n))
            .count()
    }

    fn boot(self) -> Self {
        CubeCycle(self).nth(5).unwrap()
    }
}

struct CubeCycle<D: Dimension>(Cube<D>);

impl<D: Dimension> Iterator for CubeCycle<D> {
    type Item = Cube<D>;

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
            .flat_map(Point::neighbors)
            .filter(|p| !self.0.points.contains(p));
        let add = inactive.filter(|p| self.0.active_around(p) == 3);
        self.0.points = keep.cloned().chain(add).collect();
        Some(self.0.clone())
    }
}

fn solve<D: Dimension>(cube: Cube<D>) -> usize {
    cube.boot().active()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p17.txt");
    let cube = input.parse::<Cube<D3>>()?;
    let out1 = solve(cube);
    let cube = input.parse::<Cube<D4>>()?;
    let out2 = solve(cube);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let p: Point<D3> = Point::new2(0, 0);
        assert_eq!(p.ps.len(), 3);
        assert_eq!(p.neighbors().len(), 3usize.pow(3) - 1);
        let p: Point<D4> = Point::new2(0, 0);
        assert_eq!(p.ps.len(), 4);
        assert_eq!(p.neighbors().len(), 3usize.pow(4) - 1);
    }

    #[test]
    fn test01() {
        let cube = ".#.\n\
                    ..#\n\
                    ###"
        .parse::<Cube<D3>>()
        .unwrap();
        assert_eq!(solve(cube), 112);
    }

    #[ignore]
    #[test]
    fn test02() {
        let cube = ".#.\n\
                    ..#\n\
                    ###"
        .parse::<Cube<D4>>()
        .unwrap();
        assert_eq!(solve(cube), 848);
    }
}
