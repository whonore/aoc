use std::iter;
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
    fn opp(&self) -> Self {
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
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
        Ok(Tile { id, pix })
    }
}

impl Tile {
    fn variants(&self) -> Vec<Self> {
        // TODO: lots of unnecessary cloning
        vec![
            self.clone(),
            self.flip(),
            self.rotate(),
            self.rotate().flip(),
            self.rotate().rotate(),
            self.rotate().rotate().flip(),
            self.rotate().rotate().rotate(),
            self.rotate().rotate().rotate().flip(),
        ]
    }

    fn rotate(&self) -> Self {
        let mut tile = self.clone();
        let width = tile.pix[0].len();
        let mut pix = tile.pix.clone();
        for (r, row) in tile.pix.iter().enumerate() {
            for (c, p) in row.iter().enumerate() {
                pix[c][width - 1 - r] = *p;
            }
        }
        tile.pix = pix;
        tile
    }

    fn flip(&self) -> Self {
        let mut tile = self.clone();
        for row in tile.pix.iter_mut() {
            row.reverse();
        }
        tile
    }

    fn border(&self, side: Side) -> Vec<Pixel> {
        match side {
            Top => self.pix[0].clone(),
            Bottom => self.pix.last().unwrap().to_vec(),
            Left => self.pix.iter().map(|row| row[0]).collect::<Vec<_>>(),
            Right => self
                .pix
                .iter()
                .map(|row| *row.last().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn aligns(&self, tile: &Tile, side: Side) -> bool {
        self.border(side) == tile.border(side.opp())
    }
}

#[derive(Debug)]
struct TileGrid(Vec<Vec<Tile>>);

impl TileGrid {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn new(tiles: &[Tile]) -> Self {
        let size = (tiles.len() as f64).sqrt() as usize;
        let tiles = tiles
            .iter()
            .flat_map(|tile| tile.variants())
            .collect::<Vec<_>>();
        let mut init = TileGrid(
            iter::repeat(iter::repeat(Default::default()).take(size).collect())
                .take(size)
                .collect(),
        );
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

fn solve(tiles: &[Tile]) -> u64 {
    let grid = TileGrid::new(tiles);
    let nw = &grid.0[0][0];
    let ne = &grid.0[0][grid.size() - 1];
    let se = &grid.0[grid.size() - 1][grid.size() - 1];
    let sw = &grid.0[grid.size() - 1][0];
    nw.id * ne.id * se.id * sw.id
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p20.txt");
    let tiles = input
        .split("\n\n")
        .map(|tile| tile.parse::<Tile>())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&tiles);
    let out2 = "";
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
        assert_eq!(tile, tile.rotate().rotate().rotate().rotate());
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
        assert_eq!(tile1.flip(), tile2);
        assert_eq!(tile1, tile1.flip().flip());
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
        assert_eq!(solve(&tiles), 20899048083289);
    }
}
