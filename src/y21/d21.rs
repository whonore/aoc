use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Game {
    positions: [u64; 2],
    scores: [u64; 2],
    max_score: u64,
}

impl Game {
    const fn new(max_score: u64, p1: u64, p2: u64) -> Self {
        Self {
            positions: [p1 - 1, p2 - 1],
            scores: [0; 2],
            max_score,
        }
    }

    fn add(&mut self, player: usize, roll: u64) {
        self.positions[player] = (self.positions[player] + roll) % 10;
        self.scores[player] += self.positions[player] + 1;
    }

    fn won(&self) -> bool {
        self.scores.iter().any(|&score| score >= self.max_score)
    }
}

#[derive(Debug, Clone)]
struct Games(HashMap<Game, u64>);

impl Games {
    fn new(max_score: u64, p1: u64, p2: u64) -> Self {
        Self(
            [(Game::new(max_score, p1, p2), 1)]
                .iter()
                .copied()
                .collect(),
        )
    }

    fn add(&mut self, player: usize, rolls: &[u64]) {
        let mut new = HashMap::new();
        for (game, cnt) in &self.0 {
            if game.won() {
                *new.entry(*game).or_insert(0) += cnt;
            } else {
                for roll in rolls {
                    let mut game = *game;
                    game.add(player, *roll);
                    *new.entry(game).or_insert(0) += cnt;
                }
            }
        }
        self.0 = new;
    }

    fn unfinished(&self) -> bool {
        !self.0.keys().all(Game::won)
    }

    fn wins(&self, player: usize) -> u64 {
        self.0
            .iter()
            .filter_map(|(game, cnt)| {
                (game.scores[player] > game.scores[(player + 1) % 2]).then(|| cnt)
            })
            .sum()
    }
}

fn part1(p1: u64, p2: u64) -> u64 {
    let dice = (1..=100).cycle().chunks(3);
    let mut dice = dice.into_iter();
    let mut game = Game::new(1000, p1, p2);
    let mut player = 0;
    let mut rolls = 0;
    while !game.won() {
        game.add(player, dice.next().unwrap().sum::<u64>());
        player = (player + 1) % 2;
        rolls += 3;
    }
    game.scores.iter().min().unwrap() * rolls
}

fn part2(p1: u64, p2: u64) -> [u64; 2] {
    let mut games = Games::new(21, p1, p2);
    let mut player = 0;
    let rolls = iter::repeat(1..=3)
        .take(3)
        .multi_cartesian_product()
        .map(|xs| xs.iter().sum())
        .collect::<Vec<_>>();
    while games.unfinished() {
        games.add(player, &rolls);
        player = (player + 1) % 2;
    }
    [games.wins(0), games.wins(1)]
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d21.txt");
    let (p1, p2) = {
        let parse = |line: &str| u64::from(line.chars().last().unwrap().to_digit(10).unwrap());
        let mut lines = input.lines();
        (parse(lines.next().unwrap()), parse(lines.next().unwrap()))
    };
    let out1 = part1(p1, p2);
    let out2 = part2(p1, p2);
    Ok(format!("{} {:?}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(part1(4, 8), 739_785);
    }

    #[test]
    fn test02() {
        assert_eq!(part2(4, 8), [444_356_092_776_315, 341_960_390_180_808]);
    }
}
