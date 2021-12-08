use std::cmp::Ordering;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(pt: &str) -> Result<Self, Self::Err> {
        let mut pts = pt.split(',').map(str::parse);
        let x = pts.next().unwrap().map_err(|_| "Invalid point x")?;
        let y = pts.next().unwrap().map_err(|_| "Invalid point y")?;
        Ok(Self { x, y })
    }
}

impl Point {
    #[cfg(test)]
    const fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct LineIter {
    cur: Point,
    end: Point,
    done: bool,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut pts = line.split(" -> ").map(str::parse);
        let start = pts.next().unwrap()?;
        let end = pts.next().unwrap()?;
        Ok(Self { start, end })
    }
}

impl Line {
    #[cfg(test)]
    const fn new_xy(x1: u64, y1: u64, x2: u64, y2: u64) -> Self {
        Self {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }

    const fn iter(self) -> LineIter {
        LineIter {
            cur: self.start,
            end: self.end,
            done: false,
        }
    }

    const fn horizontal(self) -> bool {
        self.start.y == self.end.y
    }

    const fn vertical(self) -> bool {
        self.start.x == self.end.x
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let pt = self.cur;
            if self.cur == self.end {
                self.done = true;
            } else {
                match self.cur.x.cmp(&self.end.x) {
                    Ordering::Less => {
                        self.cur.x += 1;
                    }
                    Ordering::Greater => {
                        self.cur.x -= 1;
                    }
                    Ordering::Equal => {}
                };
                match self.cur.y.cmp(&self.end.y) {
                    Ordering::Less => {
                        self.cur.y += 1;
                    }
                    Ordering::Greater => {
                        self.cur.y -= 1;
                    }
                    Ordering::Equal => {}
                };
            }
            Some(pt)
        }
    }
}

fn count_overlap<'a>(lines: impl IntoIterator<Item = &'a Line>) -> usize {
    lines
        .into_iter()
        .flat_map(|line| line.iter())
        .counts()
        .values()
        .filter(|&&cnt| cnt > 1)
        .count()
}

fn part1(lines: &[Line]) -> usize {
    count_overlap(
        lines
            .iter()
            .filter(|line| line.horizontal() || line.vertical()),
    )
}

fn part2(lines: &[Line]) -> usize {
    count_overlap(lines)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d05.txt");
    let lines = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&lines);
    let out2 = part2(&lines);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let lines = [
            Line::new_xy(0, 9, 5, 9),
            Line::new_xy(8, 0, 0, 8),
            Line::new_xy(9, 4, 3, 4),
            Line::new_xy(2, 2, 2, 1),
            Line::new_xy(7, 0, 7, 4),
            Line::new_xy(6, 4, 2, 0),
            Line::new_xy(0, 9, 2, 9),
            Line::new_xy(3, 4, 1, 4),
            Line::new_xy(0, 0, 8, 8),
            Line::new_xy(5, 5, 8, 2),
        ];
        assert_eq!(part1(&lines), 5);
    }

    #[test]
    fn test02() {
        let lines = [
            Line::new_xy(0, 9, 5, 9),
            Line::new_xy(8, 0, 0, 8),
            Line::new_xy(9, 4, 3, 4),
            Line::new_xy(2, 2, 2, 1),
            Line::new_xy(7, 0, 7, 4),
            Line::new_xy(6, 4, 2, 0),
            Line::new_xy(0, 9, 2, 9),
            Line::new_xy(3, 4, 1, 4),
            Line::new_xy(0, 0, 8, 8),
            Line::new_xy(5, 5, 8, 2),
        ];
        assert_eq!(part2(&lines), 12);
    }
}
