use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::AddAssign;
use std::str::FromStr;

fn counter<A, I>(xs: I) -> HashMap<A, usize>
where
    A: Clone + PartialEq + Eq + Hash,
    I: Iterator<Item = A>,
{
    let xs = xs.collect::<Vec<_>>();
    xs.iter()
        .cloned()
        .zip(xs.iter().map(|x| xs.iter().filter(|y| x == *y).count()))
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Location {
    NE(usize),
    E(usize),
    SE(usize),
    SW(usize),
    W(usize),
    NW(usize),
}
use Location::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Location> for Coord {
    fn add_assign(&mut self, loc: Location) {
        match loc {
            NE(amt) => {
                self.x += amt as isize;
                self.z -= amt as isize;
            }
            E(amt) => {
                self.x += amt as isize;
                self.y -= amt as isize;
            }
            SE(amt) => {
                self.y -= amt as isize;
                self.z += amt as isize;
            }
            SW(amt) => {
                self.x -= amt as isize;
                self.z += amt as isize;
            }
            W(amt) => {
                self.x -= amt as isize;
                self.y += amt as isize;
            }
            NW(amt) => {
                self.y += amt as isize;
                self.z -= amt as isize;
            }
        }
    }
}

impl FromStr for Coord {
    type Err = String;

    fn from_str(loc: &str) -> Result<Self, Self::Err> {
        let mut coord = Self { x: 0, y: 0, z: 0 };
        let locs = loc
            .replace("ne", "NE")
            .replace("se", "SE")
            .replace("sw", "SW")
            .replace("nw", "NW");
        coord += NE(locs.matches("NE").count());
        coord += E(locs.matches('e').count());
        coord += SE(locs.matches("SE").count());
        coord += SW(locs.matches("SW").count());
        coord += W(locs.matches('w').count());
        coord += NW(locs.matches("NW").count());
        Ok(coord)
    }
}

impl Coord {
    const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    const fn neighbors(&self) -> [Self; 6] {
        [
            Self::new(self.x + 1, self.y, self.z - 1),
            Self::new(self.x + 1, self.y - 1, self.z),
            Self::new(self.x, self.y - 1, self.z + 1),
            Self::new(self.x - 1, self.y, self.z + 1),
            Self::new(self.x - 1, self.y + 1, self.z),
            Self::new(self.x, self.y + 1, self.z - 1),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tiles(HashSet<Coord>);

impl Tiles {
    fn new(coords: &[Coord]) -> Self {
        Self(
            counter(coords.iter())
                .iter()
                .filter_map(|(coord, nflip)| if *nflip % 2 == 1 { Some(**coord) } else { None })
                .collect(),
        )
    }

    fn count(&self) -> usize {
        self.0.len()
    }

    fn day(&mut self) {
        let towhite = self
            .0
            .iter()
            .filter(|coord| {
                let neighs = coord
                    .neighbors()
                    .iter()
                    .filter(|neigh| self.0.contains(neigh))
                    .count();
                neighs == 0 || 2 < neighs
            })
            .copied()
            .collect::<HashSet<_>>();
        let toblack = self
            .0
            .iter()
            .flat_map(|coord| coord.neighbors().to_vec())
            .filter(|coord| {
                coord
                    .neighbors()
                    .iter()
                    .filter(|neigh| self.0.contains(neigh))
                    .count()
                    == 2
            })
            .collect::<HashSet<_>>();
        self.0 = self
            .0
            .difference(&towhite)
            .copied()
            .collect::<HashSet<_>>()
            .union(&toblack)
            .copied()
            .collect();
    }
}

fn solve(coords: &[Coord]) -> (usize, usize) {
    let mut tiles = Tiles::new(coords);
    let start = tiles.count();
    for _ in 0..100 {
        tiles.day();
    }
    (start, tiles.count())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d24.txt");
    let locs = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let (out1, out2) = solve(&locs);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let locs = [
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
        .iter()
        .map(|loc| loc.parse::<Coord>().unwrap())
        .collect::<Vec<_>>();
        assert_eq!(solve(&locs), (10, 2208));
    }
}
