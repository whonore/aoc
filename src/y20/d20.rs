use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pixel {
    On,
    Off,
}
use Pixel::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}
use Side::*;

impl Side {
    const fn opp(self) -> Self {
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
}

struct TileTransformer {
    tile: Tile,
    rotates: u8,
    flipped: bool,
}

impl TileTransformer {
    fn new(tile: &Tile) -> Self {
        Self {
            tile: tile.clone(),
            rotates: 0,
            flipped: false,
        }
    }

    const fn rotate(mut self, n: u8) -> Self {
        self.rotates = (self.rotates + n) % 4;
        self
    }

    const fn flip(mut self) -> Self {
        self.flipped = !self.flipped;
        self
    }

    fn build(mut self) -> Tile {
        for _ in 0..self.rotates {
            let height = self.tile.pix.len();
            let width = self.tile.pix[0].len();
            let mut pix = vec![vec![Off; height]; width];
            for (r, row) in self.tile.pix.iter().enumerate() {
                for (c, p) in row.iter().enumerate() {
                    pix[c][height - 1 - r] = *p;
                }
            }
            self.tile.pix = pix;
        }
        if self.flipped {
            for row in &mut self.tile.pix {
                row.reverse();
            }
        }
        self.tile
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Tile {
    id: u64,
    pix: Vec<Vec<Pixel>>,
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(tile: &str) -> Result<Self, Self::Err> {
        let mut lines = tile.lines();
        let id = lines
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let pix = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Ok(On),
                        '.' => Ok(Off),
                        _ => Err(format!("Invalid char {}", c)),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { id, pix })
    }
}

impl Tile {
    fn variants(&self) -> [Self; 8] {
        [
            self.clone(),
            self.trans().flip().build(),
            self.trans().rotate(1).build(),
            self.trans().rotate(1).flip().build(),
            self.trans().rotate(2).build(),
            self.trans().rotate(2).flip().build(),
            self.trans().rotate(3).build(),
            self.trans().rotate(3).flip().build(),
        ]
    }

    fn trans(&self) -> TileTransformer {
        TileTransformer::new(self)
    }

    fn border(&self, side: Side) -> Vec<Pixel> {
        match side {
            Top => self.pix[0].clone(),
            Bottom => self.pix.last().unwrap().clone(),
            Left => self.pix.iter().map(|row| row[0]).collect::<Vec<_>>(),
            Right => self
                .pix
                .iter()
                .map(|row| *row.last().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn aligns(&self, tile: &Self, side: Side) -> bool {
        self.border(side) == tile.border(side.opp())
    }

    fn strip_borders(&self) -> Self {
        let pix = self.pix[1..self.pix.len() - 1]
            .iter()
            .map(|row| row[1..row.len() - 1].to_vec())
            .collect();
        Self { id: 0, pix }
    }

    fn join(&self, tile: &Self, side: Side) -> Self {
        let mut out = self.clone();
        let mut tile = tile.clone();
        match side {
            Right => {
                for (r, row) in out.pix.iter_mut().enumerate() {
                    row.append(&mut tile.pix[r]);
                }
            }
            Bottom => out.pix.append(&mut tile.pix),
            _ => panic!("join not implemented for {:?}", side),
        }
        out
    }
}

#[derive(Debug)]
struct TileGrid(Vec<Vec<Tile>>);

impl TileGrid {
    fn size(&self) -> usize {
        self.0.len()
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    fn new(tiles: &[Tile]) -> Self {
        let size = (tiles.len() as f64).sqrt() as usize;
        let tiles = tiles
            .iter()
            .flat_map(|tile| tile.variants().to_vec())
            .collect::<Vec<_>>();
        let mut init = Self(vec![vec![Tile::default(); size]; size]);
        assert!(Self::fill_grid(&mut init, &tiles, 0, 0));
        init
    }

    fn fill_grid(grid: &mut Self, tiles: &[Tile], r: usize, c: usize) -> bool {
        if r == grid.size() {
            return true;
        }

        for tile in tiles {
            if Self::check_above(grid, tile, r, c) && Self::check_left(grid, tile, r, c) {
                grid.0[r][c] = tile.clone();
                let (rnew, cnew) = if c + 1 < grid.size() {
                    (r, c + 1)
                } else {
                    (r + 1, 0)
                };
                if Self::fill_grid(grid, tiles, rnew, cnew) {
                    return true;
                }
            }
        }
        false
    }

    fn check_above(grid: &Self, tile: &Tile, r: usize, c: usize) -> bool {
        r == 0 || {
            let above = &grid.0[r - 1][c];
            tile.id != above.id && above.aligns(tile, Bottom)
        }
    }

    fn check_left(grid: &Self, tile: &Tile, r: usize, c: usize) -> bool {
        c == 0 || {
            let left = &grid.0[r][c - 1];
            tile.id != left.id && left.aligns(tile, Right)
        }
    }
}

fn count_on(pix: &[Vec<Pixel>]) -> usize {
    pix.iter()
        .map(|row| row.iter().filter(|p| **p == On).count())
        .sum()
}

struct Image(Vec<Vec<Pixel>>);

impl Image {
    fn new(grid: &TileGrid) -> Self {
        let mut tiles = grid.0.iter().map(|row| {
            let mut tiles = row.iter().map(Tile::strip_borders);
            let first = tiles.next().unwrap();
            tiles.fold(first, |tiles, tile| tiles.join(&tile, Right))
        });
        let first = tiles.next().unwrap();
        let tiles = tiles.fold(first, |tiles, tile| tiles.join(&tile, Bottom));
        Self(tiles.pix)
    }

    fn find(&self, pattern: &Tile) -> usize {
        let mut matches = 0;
        let pheight = pattern.pix.len();
        let pwidth = pattern.pix[0].len();
        let height = self.0.len();
        let width = self.0[0].len();
        for r in 0..height - pheight {
            for c in 0..width - pwidth {
                if pattern
                    .pix
                    .iter()
                    .enumerate()
                    .all(|(pr, prow)| Self::row_match(prow, &self.0[r + pr][c..c + pwidth]))
                {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn row_match(pattern: &[Pixel], row: &[Pixel]) -> bool {
        pattern
            .iter()
            .zip(row)
            .all(|(pat, pix)| *pat == Off || pat == pix)
    }
}

fn solve(tiles: &[Tile]) -> (u64, usize) {
    let monster = "Tile 0:\n\
                   ..................#.\n\
                   #....##....##....###\n\
                   .#..#..#..#..#..#..."
        .parse::<Tile>()
        .unwrap();
    let monsters = monster.variants();
    let grid = TileGrid::new(tiles);
    let nw = &grid.0[0][0];
    let ne = &grid.0[0][grid.size() - 1];
    let se = &grid.0[grid.size() - 1][grid.size() - 1];
    let sw = &grid.0[grid.size() - 1][0];
    let img = Image::new(&grid);
    let nmonsters: usize = monsters.iter().map(|m| img.find(m)).sum();
    (
        nw.id * ne.id * se.id * sw.id,
        count_on(&img.0) - nmonsters * count_on(&monster.pix),
    )
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d20.txt");
    let tiles = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let (out1, out2) = solve(&tiles);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border() {
        let tile = "Tile 1:\n\
                    .#.#\n\
                    #...\n\
                    #..#\n\
                    ####"
            .parse::<Tile>()
            .unwrap();
        assert_eq!(tile.border(Top), [Off, On, Off, On]);
        assert_eq!(tile.border(Bottom), [On, On, On, On]);
        assert_eq!(tile.border(Left), [Off, On, On, On]);
        assert_eq!(tile.border(Right), [On, Off, On, On]);
    }

    #[test]
    fn test_rotate() {
        let tile = "Tile 1:\n\
                    .#.#\n\
                    #...\n\
                    #..#\n\
                    ####"
            .parse::<Tile>()
            .unwrap();
        assert_eq!(tile, tile.trans().rotate(4).build());
    }

    #[test]
    fn test_flip() {
        let tile1 = "Tile 1:\n\
                     .#.#\n\
                     #...\n\
                     #..#\n\
                     ####"
            .parse::<Tile>()
            .unwrap();
        let tile2 = "Tile 1:\n\
                     #.#.\n\
                     ...#\n\
                     #..#\n\
                     ####"
            .parse::<Tile>()
            .unwrap();
        assert_eq!(tile1.trans().flip().build(), tile2);
        assert_eq!(tile1, tile1.trans().flip().flip().build());
    }

    #[test]
    fn test01() {
        let tiles = [
            "Tile 2311:\n\
             ..##.#..#.\n\
             ##..#.....\n\
             #...##..#.\n\
             ####.#...#\n\
             ##.##.###.\n\
             ##...#.###\n\
             .#.#.#..##\n\
             ..#....#..\n\
             ###...#.#.\n\
             ..###..###",
            "Tile 1951:\n\
             #.##...##.\n\
             #.####...#\n\
             .....#..##\n\
             #...######\n\
             .##.#....#\n\
             .###.#####\n\
             ###.##.##.\n\
             .###....#.\n\
             ..#.#..#.#\n\
             #...##.#..",
            "Tile 1171:\n\
             ####...##.\n\
             #..##.#..#\n\
             ##.#..#.#.\n\
             .###.####.\n\
             ..###.####\n\
             .##....##.\n\
             .#...####.\n\
             #.##.####.\n\
             ####..#...\n\
             .....##...",
            "Tile 1427:\n\
             ###.##.#..\n\
             .#..#.##..\n\
             .#.##.#..#\n\
             #.#.#.##.#\n\
             ....#...##\n\
             ...##..##.\n\
             ...#.#####\n\
             .#.####.#.\n\
             ..#..###.#\n\
             ..##.#..#.",
            "Tile 1489:\n\
             ##.#.#....\n\
             ..##...#..\n\
             .##..##...\n\
             ..#...#...\n\
             #####...#.\n\
             #..#.#.#.#\n\
             ...#.#.#..\n\
             ##.#...##.\n\
             ..##.##.##\n\
             ###.##.#..",
            "Tile 2473:\n\
             #....####.\n\
             #..#.##...\n\
             #.##..#...\n\
             ######.#.#\n\
             .#...#.#.#\n\
             .#########\n\
             .###.#..#.\n\
             ########.#\n\
             ##...##.#.\n\
             ..###.#.#.",
            "Tile 2971:\n\
             ..#.#....#\n\
             #...###...\n\
             #.#.###...\n\
             ##.##..#..\n\
             .#####..##\n\
             .#..####.#\n\
             #..#.#..#.\n\
             ..####.###\n\
             ..#.#.###.\n\
             ...#.#.#.#",
            "Tile 2729:\n\
             ...#.#.#.#\n\
             ####.#....\n\
             ..#.#.....\n\
             ....#..#.#\n\
             .##..##.#.\n\
             .#.####...\n\
             ####.#.#..\n\
             ##.####...\n\
             ##..#.##..\n\
             #.##...##.",
            "Tile 3079:\n\
             #.#.#####.\n\
             .#..######\n\
             ..#.......\n\
             ######....\n\
             ####.#..#.\n\
             .#...#.##.\n\
             #.#####.##\n\
             ..#.###...\n\
             ..#.......\n\
             ..#.###...",
        ]
        .iter()
        .map(|tile| tile.parse::<Tile>().unwrap())
        .collect::<Vec<_>>();
        assert_eq!(solve(&tiles), (20_899_048_083_289, 273));
    }
}
