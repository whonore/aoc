use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}
use Seat::*;

impl Seat {
    fn flip(&self) -> Self {
        match self {
            Floor => Floor,
            Empty => Occupied,
            Occupied => Empty,
        }
    }
}

struct Grid(Vec<Vec<Seat>>);

impl FromStr for Grid {
    type Err = String;

    fn from_str(grid: &str) -> Result<Self, Self::Err> {
        let grid = grid
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Ok(Floor),
                        'L' => Ok(Empty),
                        '#' => Ok(Occupied),
                        _ => Err(format!("Invalid seat {}", c)),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        Ok(Grid(grid))
    }
}

impl Grid {
    fn step(&mut self) -> bool {
        let mut flip: Vec<(usize, usize)> = vec![];
        for (r, row) in self.0.iter().enumerate() {
            for (c, seat) in row.iter().enumerate() {
                match seat {
                    Empty => {
                        if self.count_adj(r, c) == 0 {
                            flip.push((r, c));
                        }
                    }
                    Occupied => {
                        if 4 <= self.count_adj(r, c) {
                            flip.push((r, c));
                        }
                    }
                    _ => {}
                }
            }
        }
        for (r, c) in &flip {
            self.0[*r][*c] = self.0[*r][*c].flip();
        }
        !flip.is_empty()
    }

    fn stabilize(&mut self) {
        while self.step() {}
    }

    fn count_adj(&self, r: usize, c: usize) -> usize {
        [
            r.checked_sub(1)
                .and_then(|r| self.0.get(r))
                .and_then(|row| c.checked_sub(1).and_then(|c| row.get(c))),
            r.checked_sub(1)
                .and_then(|r| self.0.get(r))
                .and_then(|row| row.get(c)),
            r.checked_sub(1)
                .and_then(|r| self.0.get(r))
                .and_then(|row| row.get(c + 1)),
            self.0
                .get(r)
                .and_then(|row| c.checked_sub(1).and_then(|c| row.get(c))),
            self.0.get(r).and_then(|row| row.get(c + 1)),
            self.0
                .get(r + 1)
                .and_then(|row| c.checked_sub(1).and_then(|c| row.get(c))),
            self.0.get(r + 1).and_then(|row| row.get(c)),
            self.0.get(r + 1).and_then(|row| row.get(c + 1)),
        ]
        .iter()
        .filter(|seat| seat.map(|s| *s == Occupied).unwrap_or(false))
        .count()
    }

    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|seat| **seat == Occupied).count())
            .sum()
    }
}

fn solve(grid: &mut Grid) -> usize {
    grid.stabilize();
    grid.count_occupied()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p11.txt");
    let mut grid = input.parse::<Grid>()?;
    let out1 = solve(&mut grid);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut grid = "L.LL.LL.LL\n\
                        LLLLLLL.LL\n\
                        L.L.L..L..\n\
                        LLLL.LL.LL\n\
                        L.LL.LL.LL\n\
                        L.LLLLL.LL\n\
                        ..L.L.....\n\
                        LLLLLLLLLL\n\
                        L.LLLLLL.L\n\
                        L.LLLLL.LL"
            .parse::<Grid>()
            .unwrap();
        assert_eq!(solve(&mut grid), 37);
    }
}
