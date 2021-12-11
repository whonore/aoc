use std::str::FromStr;

fn neighbors(r: usize, c: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let up = r.checked_sub(1);
    let down = (r + 1 < h).then(|| r + 1);
    let left = c.checked_sub(1);
    let right = (c + 1 < w).then(|| c + 1);
    [
        up.zip(Some(c)),
        up.zip(left),
        up.zip(right),
        down.zip(Some(c)),
        down.zip(left),
        down.zip(right),
        Some(r).zip(left),
        Some(r).zip(right),
    ]
    .iter()
    .copied()
    .flatten()
    .collect()
}

#[derive(Debug, Clone, Copy)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    const fn new(energy: u32) -> Self {
        Self {
            energy,
            flashed: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Octopus>>);

impl FromStr for Grid {
    type Err = String;

    fn from_str(grid: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            grid.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            c.to_digit(10)
                                .map(Octopus::new)
                                .ok_or_else(|| format!("Invalid char: {}", c))
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Grid {
    fn step(&mut self) -> usize {
        let mut flashes = 0;
        for row in &mut self.0 {
            for oct in row.iter_mut() {
                oct.energy += 1;
                oct.flashed = false;
            }
        }

        let h = self.0.len();
        let w = self.0[0].len();
        loop {
            let mut flashed = false;
            let mut to_bump = vec![];
            for (r, row) in self.0.iter_mut().enumerate() {
                for (c, oct) in row.iter_mut().enumerate() {
                    if oct.energy > 9 && !oct.flashed {
                        oct.flashed = true;
                        oct.energy = 0;
                        flashes += 1;
                        flashed = true;
                        to_bump.extend(neighbors(r, c, w, h));
                    }
                }
            }
            for (r, c) in to_bump {
                if !self.0[r][c].flashed {
                    self.0[r][c].energy += 1;
                }
            }

            if !flashed {
                break;
            }
        }

        flashes
    }
}

fn part1(mut grid: Grid, steps: u64) -> usize {
    (0..steps).map(|_| grid.step()).sum()
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d11.txt");
    let grid = input.parse()?;
    let out1 = part1(grid, 100);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let grid = "5483143223\n\
                    2745854711\n\
                    5264556173\n\
                    6141336146\n\
                    6357385478\n\
                    4167524645\n\
                    2176841721\n\
                    6882881134\n\
                    4846848554\n\
                    5283751526"
            .parse::<Grid>()
            .unwrap();
        assert_eq!(part1(grid.clone(), 10), 204);
        assert_eq!(part1(grid, 100), 1656);
    }
}
