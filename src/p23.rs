struct Game {
    cups: Vec<usize>,
}

impl Game {
    fn new(cups: &[usize]) -> Self {
        Self {
            cups: cups.to_vec(),
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tomove = self.cups.drain(1..=3).collect::<Vec<_>>();

        let mut dest = self.cups[0] - 1;
        while !self.cups.contains(&dest) {
            dest = if dest <= 1 {
                *self.cups.iter().max().unwrap()
            } else {
                dest - 1
            };
        }
        let dest_idx = self.cups.iter().position(|cup| *cup == dest).unwrap();

        for cup in tomove.drain(..).rev() {
            self.cups.insert(dest_idx + 1, cup);
        }
        self.cups.rotate_left(1);

        let (pre, post) = self
            .cups
            .split_at(self.cups.iter().position(|cup| *cup == 1).unwrap());
        Some(
            post[1..]
                .iter()
                .chain(pre)
                .fold(0, |acc, cup| acc * 10 + cup),
        )
    }
}

fn solve(cups: &[usize], moves: usize) -> usize {
    Game::new(cups).nth(moves - 1).unwrap()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p23.txt");
    let cups = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(&cups, 100);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(solve(&cups, 10), 92658374);
        assert_eq!(solve(&cups, 100), 67384529);
    }
}
