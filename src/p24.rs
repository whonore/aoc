use std::collections::HashMap;
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
        let mut coord = Coord { x: 0, y: 0, z: 0 };
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

fn solve(coords: &[Coord]) -> usize {
    counter(coords.iter())
        .iter()
        .filter(|(_, nflip)| **nflip % 2 == 1)
        .count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p24.txt");
    let locs = input
        .lines()
        .map(|x| x.parse::<Coord>())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&locs);
    let out2 = "";
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
        assert_eq!(solve(&locs), 10);
    }
}
