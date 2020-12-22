use std::collections::VecDeque;

type Card = usize;
type Deck = VecDeque<Card>;

fn round(deck1: &mut Deck, deck2: &mut Deck) {
    let c1 = deck1.pop_front().unwrap();
    let c2 = deck2.pop_front().unwrap();
    if c1 < c2 {
        deck2.push_back(c2);
        deck2.push_back(c1);
    } else {
        deck1.push_back(c1);
        deck1.push_back(c2);
    }
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, c)| (idx + 1) * c)
        .sum()
}

fn solve(deck1: &mut Deck, deck2: &mut Deck) -> usize {
    while !(deck1.is_empty() || deck2.is_empty()) {
        round(deck1, deck2)
    }
    score(deck1) + score(deck2)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p22.txt");
    let mut decks = input.split("\n\n").map(|deck| {
        deck.lines()
            .skip(1)
            .map(|c| c.parse::<Card>().unwrap())
            .collect::<Deck>()
    });
    let mut deck1 = decks.next().unwrap();
    let mut deck2 = decks.next().unwrap();
    let out1 = solve(&mut deck1, &mut deck2);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut deck1 = [9, 2, 6, 3, 1].iter().cloned().collect::<Deck>();
        let mut deck2 = [5, 8, 4, 7, 10].iter().cloned().collect::<Deck>();
        assert_eq!(solve(&mut deck1, &mut deck2), 306);
    }
}
