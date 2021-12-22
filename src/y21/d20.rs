use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Light,
    Dark,
}

impl TryFrom<char> for Pixel {
    type Error = char;

    fn try_from(pix: char) -> Result<Self, Self::Error> {
        match pix {
            '#' => Ok(Self::Light),
            '.' => Ok(Self::Dark),
            _ => Err(pix),
        }
    }
}

impl From<Pixel> for usize {
    fn from(pix: Pixel) -> Self {
        match pix {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
}

impl Pixel {
    const fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Debug, Clone)]
struct Algorithm {
    pixels: [Pixel; 512],
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(alg: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pixels: alg
                .chars()
                .map(Pixel::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|c| format!("Invalid pixel: {}", c))?
                .try_into()
                .map_err(|_| "Invalid algorithm length")?,
        })
    }
}

impl Algorithm {
    fn get_pixel(&self, pix: &[Pixel]) -> Pixel {
        let n = pix
            .iter()
            .fold(0, |acc, &pix| (acc << 1) | usize::from(pix));
        self.pixels[n]
    }

    fn toggles(&self) -> bool {
        self.pixels[0] == Pixel::Light
    }
}

#[derive(Debug, Clone)]
struct Image {
    bg: Pixel,
    pixels: HashMap<(isize, isize), Pixel>,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(img: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pixels: img
                .lines()
                .enumerate()
                .flat_map(|(r, line)| {
                    line.chars().enumerate().map(move |(c, pix)| {
                        #[allow(clippy::cast_possible_wrap)]
                        Pixel::try_from(pix).map(|pix| ((r as isize, c as isize), pix))
                    })
                })
                .collect::<Result<HashMap<_, _>, _>>()
                .map_err(|c| format!("Invalid pixel: {}", c))?,
            bg: Pixel::Dark,
        })
    }
}

impl Image {
    fn enhance(&mut self, alg: &Algorithm) {
        self.pixels = self
            .pixels
            .keys()
            .flat_map(|(r, c)| {
                (r - 2..r + 2).flat_map(move |r2| (c - 2..c + 2).map(move |c2| (r2, c2)))
            })
            .map(|(r, c)| ((r, c), alg.get_pixel(&self.neighbors(r, c))))
            .collect();
        if alg.toggles() {
            self.bg = self.bg.toggle();
        }
    }

    fn neighbors(&self, r: isize, c: isize) -> [Pixel; 9] {
        let up = r - 1;
        let down = r + 1;
        let left = c - 1;
        let right = c + 1;
        [
            (up, left),
            (up, c),
            (up, right),
            (r, left),
            (r, c),
            (r, right),
            (down, left),
            (down, c),
            (down, right),
        ]
        .map(|(r, c)| self.pixels.get(&(r, c)).copied().unwrap_or(self.bg))
    }

    fn count(&self) -> usize {
        self.pixels
            .values()
            .filter(|&&pix| pix == Pixel::Light)
            .count()
    }
}

fn part1(mut img: Image, alg: &Algorithm) -> usize {
    for _ in 0..2 {
        img.enhance(alg);
    }
    img.count()
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d20.txt");
    let (alg, img) = {
        let mut blocks = input.split("\n\n");
        (
            blocks.next().unwrap().parse()?,
            blocks.next().unwrap().parse()?,
        )
    };
    let out1 = part1(img, &alg);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let alg = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#".parse().unwrap();
        let img = "#..#.\n\
                   #....\n\
                   ##..#\n\
                   ..#..\n\
                   ..###"
            .parse()
            .unwrap();
        assert_eq!(part1(img, &alg), 35);
    }
}
