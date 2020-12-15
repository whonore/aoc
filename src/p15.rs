use std::collections::HashMap;

struct Seq {
    rounds: HashMap<usize, (usize, usize)>,
    turn: usize,
    last: usize,
}

impl Seq {
    fn new(seq: &[usize]) -> Self {
        Seq {
            rounds: seq.iter().copied().zip((1..).zip(1..)).collect(),
            turn: seq.len(),
            last: *seq.last().unwrap(),
        }
    }
}

impl Iterator for Seq {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.turn += 1;
        self.last = self
            .rounds
            .get(&self.last)
            .map(|(rnd1, rnd2)| rnd2 - rnd1)
            .unwrap_or(0);
        let (_, rnd) = self
            .rounds
            .get(&self.last)
            .copied()
            .unwrap_or((0, self.turn));
        self.rounds.insert(self.last, (rnd, self.turn));
        Some(self.last)
    }
}

fn solve(seq: &[usize], n: usize) -> usize {
    let (mut seq, n) = (Seq::new(seq), n - seq.len() - 1);
    seq.nth(n).unwrap()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p15.txt");
    let seq = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(&seq, 2020);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(&[0, 3, 6], 10), 0);
        assert_eq!(solve(&[1, 3, 2], 2020), 1);
        assert_eq!(solve(&[2, 1, 3], 2020), 10);
        assert_eq!(solve(&[1, 2, 3], 2020), 27);
        assert_eq!(solve(&[2, 3, 1], 2020), 78);
        assert_eq!(solve(&[3, 2, 1], 2020), 438);
        assert_eq!(solve(&[3, 1, 2], 2020), 1836);
    }
}
