use std::collections::{HashSet, VecDeque};

type Card = usize;
type Deck = VecDeque<Card>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    P1,
    P2,
    Infinite,
}
use Player::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mode {
    Regular,
    Recursive,
}
use Mode::*;

fn round(deck1: &mut Deck, deck2: &mut Deck, mode: Mode) -> Player {
    let c1 = deck1.pop_front().unwrap();
    let c2 = deck2.pop_front().unwrap();
    let winner = if mode == Recursive && c1 <= deck1.len() && c2 <= deck2.len() {
        let mut deck1 = deck1.iter().take(c1).copied().collect();
        let mut deck2 = deck2.iter().take(c2).copied().collect();
        play(&mut deck1, &mut deck2, mode)
    } else if c1 < c2 {
        P2
    } else {
        P1
    };
    match winner {
        P1 | Infinite => {
            deck1.push_back(c1);
            deck1.push_back(c2);
        }
        P2 => {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }
    winner
}

fn play(deck1: &mut Deck, deck2: &mut Deck, mode: Mode) -> Player {
    let mut prev = HashSet::new();
    while !(deck1.is_empty() || deck2.is_empty()) {
        let d = (deck1.clone(), deck2.clone());
        if prev.contains(&d) {
            return Infinite;
        } else {
            prev.insert(d);
        }
        round(deck1, deck2, mode);
    }
    if deck1.is_empty() {
        P2
    } else {
        P1
    }
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, c)| (idx + 1) * c)
        .sum()
}

fn solve(deck1: &mut Deck, deck2: &mut Deck, mode: Mode) -> usize {
    score(match play(deck1, deck2, mode) {
        P1 | Infinite => deck1,
        P2 => deck2,
    })
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d22.txt");
    let mut decks = input.split("\n\n").map(|deck| {
        deck.lines()
            .skip(1)
            .map(|c| c.parse::<Card>().unwrap())
            .collect::<Deck>()
    });
    let mut deck1 = decks.next().unwrap();
    let mut deck2 = decks.next().unwrap();
    let out1 = solve(&mut deck1.clone(), &mut deck2.clone(), Regular);
    let out2 = solve(&mut deck1, &mut deck2, Recursive);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut deck1 = [9, 2, 6, 3, 1].iter().copied().collect::<Deck>();
        let mut deck2 = [5, 8, 4, 7, 10].iter().copied().collect::<Deck>();
        assert_eq!(solve(&mut deck1, &mut deck2, Regular), 306);
    }

    #[test]
    fn test02() {
        let mut deck1 = [9, 2, 6, 3, 1].iter().copied().collect::<Deck>();
        let mut deck2 = [5, 8, 4, 7, 10].iter().copied().collect::<Deck>();
        assert_eq!(solve(&mut deck1, &mut deck2, Recursive), 291);
    }

    #[test]
    fn test_inf() {
        let mut deck1 = [43, 19].iter().copied().collect::<Deck>();
        let mut deck2 = [2, 29, 14].iter().copied().collect::<Deck>();
        assert_eq!(play(&mut deck1, &mut deck2, Recursive), Infinite);
    }
}
