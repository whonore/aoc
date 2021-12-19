use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Sub};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(pt: &str) -> Result<Self, Self::Err> {
        let mut xyz = pt.trim().split(',').map(str::parse);
        Ok(Self {
            x: xyz.next().unwrap().unwrap(),
            y: xyz.next().unwrap().unwrap(),
            z: xyz.next().unwrap().unwrap(),
        })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Point {
    fn rotate(&self, rot: u8) -> Self {
        let (x, y, z) = match rot {
            0 => (self.x, self.y, self.z),
            1 => (self.x, self.z, -self.y),
            2 => (self.x, -self.y, -self.z),
            3 => (self.x, -self.z, self.y),
            4 => (self.y, self.x, -self.z),
            5 => (self.y, self.z, self.x),
            6 => (self.y, -self.x, self.z),
            7 => (self.y, -self.z, -self.x),
            8 => (self.z, self.x, self.y),
            9 => (self.z, self.y, -self.x),
            10 => (self.z, -self.x, -self.y),
            11 => (self.z, -self.y, self.x),
            12 => (-self.x, self.y, -self.z),
            13 => (-self.x, self.z, self.y),
            14 => (-self.x, -self.y, self.z),
            15 => (-self.x, -self.z, -self.y),
            16 => (-self.y, self.x, self.z),
            17 => (-self.y, self.z, -self.x),
            18 => (-self.y, -self.x, -self.z),
            19 => (-self.y, -self.z, self.x),
            20 => (-self.z, self.x, -self.y),
            21 => (-self.z, self.y, self.x),
            22 => (-self.z, -self.x, self.y),
            23 => (-self.z, -self.y, -self.x),
            _ => unreachable!(),
        };
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: HashSet<Point>,
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(scanner: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            beacons: scanner
                .lines()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Map {
    scanners: VecDeque<Scanner>,
    beacons: HashSet<Point>,
}

impl Map {
    fn new(scanners: &[Scanner]) -> Self {
        let (first, scanners) = scanners.split_first().unwrap();
        Self {
            scanners: scanners.iter().cloned().collect(),
            beacons: first.beacons.clone(),
        }
    }

    fn resolve_all(&mut self) {
        while let Some(scanner) = self.scanners.pop_back() {
            if !Self::resolve(&mut self.beacons, &scanner) {
                self.scanners.push_front(scanner);
            }
        }
    }

    fn resolve(beacons: &mut HashSet<Point>, scanner: &Scanner) -> bool {
        (0..24).any(|r| {
            let rotated = scanner
                .beacons
                .iter()
                .map(|&pt| pt.rotate(r))
                .collect::<Vec<_>>();
            beacons
                .iter()
                .cartesian_product(&rotated)
                .map(|(p1, p2)| *p1 - *p2)
                .map(|off| rotated.iter().map(|pt| *pt + off).collect::<Vec<_>>())
                .find(|rotated_off| {
                    rotated_off.iter().filter(|pt| beacons.contains(pt)).count() >= 12
                })
                .map(|rotated_off| beacons.extend(rotated_off))
                .is_some()
        })
    }
}

fn part1(scanners: &[Scanner]) -> usize {
    let mut map = Map::new(scanners);
    map.resolve_all();
    map.beacons.len()
}

fn part2(scanners: &[Scanner]) -> u64 {
    let mut map = Map::new(scanners);
    map.resolve_all();
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d19.txt");
    let scanners = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&scanners);
    let out2 = part2(&scanners);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let scanners = [
            "--- scanner 0 ---\n\
             404,-588,-901\n\
             528,-643,409\n\
             -838,591,734\n\
             390,-675,-793\n\
             -537,-823,-458\n\
             -485,-357,347\n\
             -345,-311,381\n\
             -661,-816,-575\n\
             -876,649,763\n\
             -618,-824,-621\n\
             553,345,-567\n\
             474,580,667\n\
             -447,-329,318\n\
             -584,868,-557\n\
             544,-627,-890\n\
             564,392,-477\n\
             455,729,728\n\
             -892,524,684\n\
             -689,845,-530\n\
             423,-701,434\n\
             7,-33,-71\n\
             630,319,-379\n\
             443,580,662\n\
             -789,900,-551\n\
             459,-707,401",
            "--- scanner 1 ---\n\
             686,422,578\n\
             605,423,415\n\
             515,917,-361\n\
             -336,658,858\n\
             95,138,22\n\
             -476,619,847\n\
             -340,-569,-846\n\
             567,-361,727\n\
             -460,603,-452\n\
             669,-402,600\n\
             729,430,532\n\
             -500,-761,534\n\
             -322,571,750\n\
             -466,-666,-811\n\
             -429,-592,574\n\
             -355,545,-477\n\
             703,-491,-529\n\
             -328,-685,520\n\
             413,935,-424\n\
             -391,539,-444\n\
             586,-435,557\n\
             -364,-763,-893\n\
             807,-499,-711\n\
             755,-354,-619\n\
             553,889,-390",
            "--- scanner 2 ---\n\
             649,640,665\n\
             682,-795,504\n\
             -784,533,-524\n\
             -644,584,-595\n\
             -588,-843,648\n\
             -30,6,44\n\
             -674,560,763\n\
             500,723,-460\n\
             609,671,-379\n\
             -555,-800,653\n\
             -675,-892,-343\n\
             697,-426,-610\n\
             578,704,681\n\
             493,664,-388\n\
             -671,-858,530\n\
             -667,343,800\n\
             571,-461,-707\n\
             -138,-166,112\n\
             -889,563,-600\n\
             646,-828,498\n\
             640,759,510\n\
             -630,509,768\n\
             -681,-892,-333\n\
             673,-379,-804\n\
             -742,-814,-386\n\
             577,-820,562",
            "--- scanner 3 ---\n\
             -589,542,597\n\
             605,-692,669\n\
             -500,565,-823\n\
             -660,373,557\n\
             -458,-679,-417\n\
             -488,449,543\n\
             -626,468,-788\n\
             338,-750,-386\n\
             528,-832,-391\n\
             562,-778,733\n\
             -938,-730,414\n\
             543,643,-506\n\
             -524,371,-870\n\
             407,773,750\n\
             -104,29,83\n\
             378,-903,-323\n\
             -778,-728,485\n\
             426,699,580\n\
             -438,-605,-362\n\
             -469,-447,-387\n\
             509,732,623\n\
             647,635,-688\n\
             -868,-804,481\n\
             614,-800,639\n\
             595,780,-596",
            "--- scanner 4 ---\n\
             727,592,562\n\
             -293,-554,779\n\
             441,611,-461\n\
             -714,465,-776
             -743,427,-804\n\
             -660,-479,-426\n\
             832,-632,460\n\
             927,-485,-438\n\
             408,393,-506\n\
             466,436,-512\n\
             110,16,151\n\
             -258,-428,682\n\
             -393,719,612\n\
             -211,-452,876\n\
             808,-476,-593\n\
             -575,615,604\n\
             -485,667,467\n\
             -680,325,-822\n\
             -627,-443,-432\n\
             872,-547,-609\n\
             833,512,582\n\
             807,604,487\n\
             839,-516,451\n\
             891,-625,532\n\
             -652,-548,-490\n\
             30,-46,-14",
        ]
        .iter()
        .copied()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        assert_eq!(part1(&scanners), 79);
    }

    #[test]
    fn test02() {
        let scanners = [
            "--- scanner 0 ---\n\
             404,-588,-901\n\
             528,-643,409\n\
             -838,591,734\n\
             390,-675,-793\n\
             -537,-823,-458\n\
             -485,-357,347\n\
             -345,-311,381\n\
             -661,-816,-575\n\
             -876,649,763\n\
             -618,-824,-621\n\
             553,345,-567\n\
             474,580,667\n\
             -447,-329,318\n\
             -584,868,-557\n\
             544,-627,-890\n\
             564,392,-477\n\
             455,729,728\n\
             -892,524,684\n\
             -689,845,-530\n\
             423,-701,434\n\
             7,-33,-71\n\
             630,319,-379\n\
             443,580,662\n\
             -789,900,-551\n\
             459,-707,401",
            "--- scanner 1 ---\n\
             686,422,578\n\
             605,423,415\n\
             515,917,-361\n\
             -336,658,858\n\
             95,138,22\n\
             -476,619,847\n\
             -340,-569,-846\n\
             567,-361,727\n\
             -460,603,-452\n\
             669,-402,600\n\
             729,430,532\n\
             -500,-761,534\n\
             -322,571,750\n\
             -466,-666,-811\n\
             -429,-592,574\n\
             -355,545,-477\n\
             703,-491,-529\n\
             -328,-685,520\n\
             413,935,-424\n\
             -391,539,-444\n\
             586,-435,557\n\
             -364,-763,-893\n\
             807,-499,-711\n\
             755,-354,-619\n\
             553,889,-390",
            "--- scanner 2 ---\n\
             649,640,665\n\
             682,-795,504\n\
             -784,533,-524\n\
             -644,584,-595\n\
             -588,-843,648\n\
             -30,6,44\n\
             -674,560,763\n\
             500,723,-460\n\
             609,671,-379\n\
             -555,-800,653\n\
             -675,-892,-343\n\
             697,-426,-610\n\
             578,704,681\n\
             493,664,-388\n\
             -671,-858,530\n\
             -667,343,800\n\
             571,-461,-707\n\
             -138,-166,112\n\
             -889,563,-600\n\
             646,-828,498\n\
             640,759,510\n\
             -630,509,768\n\
             -681,-892,-333\n\
             673,-379,-804\n\
             -742,-814,-386\n\
             577,-820,562",
            "--- scanner 3 ---\n\
             -589,542,597\n\
             605,-692,669\n\
             -500,565,-823\n\
             -660,373,557\n\
             -458,-679,-417\n\
             -488,449,543\n\
             -626,468,-788\n\
             338,-750,-386\n\
             528,-832,-391\n\
             562,-778,733\n\
             -938,-730,414\n\
             543,643,-506\n\
             -524,371,-870\n\
             407,773,750\n\
             -104,29,83\n\
             378,-903,-323\n\
             -778,-728,485\n\
             426,699,580\n\
             -438,-605,-362\n\
             -469,-447,-387\n\
             509,732,623\n\
             647,635,-688\n\
             -868,-804,481\n\
             614,-800,639\n\
             595,780,-596",
            "--- scanner 4 ---\n\
             727,592,562\n\
             -293,-554,779\n\
             441,611,-461\n\
             -714,465,-776
             -743,427,-804\n\
             -660,-479,-426\n\
             832,-632,460\n\
             927,-485,-438\n\
             408,393,-506\n\
             466,436,-512\n\
             110,16,151\n\
             -258,-428,682\n\
             -393,719,612\n\
             -211,-452,876\n\
             808,-476,-593\n\
             -575,615,604\n\
             -485,667,467\n\
             -680,325,-822\n\
             -627,-443,-432\n\
             872,-547,-609\n\
             833,512,582\n\
             807,604,487\n\
             839,-516,451\n\
             891,-625,532\n\
             -652,-548,-490\n\
             30,-46,-14",
        ]
        .iter()
        .copied()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        assert_eq!(part2(&scanners), 3621);
    }
}
