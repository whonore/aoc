use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}

impl Axis {
    const fn new(c: char) -> Option<Self> {
        match c {
            'x' => Some(Self::X),
            'y' => Some(Self::Y),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Fold {
    axis: Axis,
    val: usize,
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(pt: &str) -> Result<Self, Self::Err> {
        let mut pt = pt.split('=');
        Ok(Self {
            axis: Axis::new(pt.next().unwrap().chars().last().unwrap()).ok_or("Invalid axis")?,
            val: pt.next().unwrap().parse().map_err(|_| "Invalid val")?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(pt: &str) -> Result<Self, Self::Err> {
        let mut pt = pt.split(',');
        Ok(Self {
            x: pt.next().unwrap().parse().map_err(|_| "Invalid x")?,
            y: pt.next().unwrap().parse().map_err(|_| "Invalid y")?,
        })
    }
}

impl Point {
    const fn reflect(&self, fold: &Fold) -> Self {
        let val = match fold.axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        };
        if val < fold.val {
            *self
        } else {
            let val = 2 * fold.val - val;
            match fold.axis {
                Axis::X => Self { x: val, y: self.y },
                Axis::Y => Self { x: self.x, y: val },
            }
        }
    }
}

fn draw<'p>(pts: impl Iterator<Item = &'p Point>, width: usize, height: usize) -> String {
    let mut img = vec![vec![' '; width]; height];
    for pt in pts {
        img[pt.y][pt.x] = '\u{2588}';
    }
    img.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn part1(pts: &HashSet<Point>, folds: &[Fold]) -> usize {
    pts.iter()
        .map(|pt| pt.reflect(&folds[0]))
        .collect::<HashSet<_>>()
        .len()
}

fn part2(pts: &HashSet<Point>, folds: &[Fold]) -> String {
    let pts = folds.iter().fold(pts.clone(), |pts, fold| {
        pts.iter()
            .map(|pt| pt.reflect(fold))
            .collect::<HashSet<_>>()
    });
    draw(
        pts.iter(),
        pts.iter().map(|pt| pt.x).max().unwrap_or(0) + 1,
        pts.iter().map(|pt| pt.y).max().unwrap_or(0) + 1,
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d13.txt");
    let (pts, folds) = {
        let mut fields = input.split("\n\n");
        (fields.next().unwrap(), fields.next().unwrap())
    };
    let pts = pts
        .lines()
        .map(str::parse)
        .collect::<Result<HashSet<_>, _>>()?;
    let folds = folds
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&pts, &folds);
    let out2 = part2(&pts, &folds);
    Ok(format!("{}\n{}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let pts = [
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ]
        .iter()
        .map(|&(x, y)| Point { x, y })
        .collect();
        let folds = [
            Fold {
                axis: Axis::Y,
                val: 7,
            },
            Fold {
                axis: Axis::X,
                val: 5,
            },
        ];
        assert_eq!(part1(&pts, &folds), 17);
    }
}
