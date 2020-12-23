fn list_vec(xs: &[usize]) -> Vec<usize> {
    let mut list = vec![0; xs.len() + 1];
    for pair in xs.windows(2) {
        list[pair[0]] = pair[1];
    }
    list[*xs.last().unwrap()] = xs[0];
    list
}

struct Game {
    cups: Vec<usize>,
    current: usize,
    max: usize,
}

impl Game {
    fn new(cups: &[usize], extra: Option<usize>) -> Self {
        let mut max = *cups.iter().max().unwrap();
        let mut cups = cups.to_vec();
        if let Some(extra) = extra {
            cups.append(&mut (cups.iter().max().unwrap() + 1..=extra).collect());
            max = extra;
        }
        Self {
            cups: list_vec(&cups),
            current: cups[0],
            max,
        }
    }

    fn play(&mut self) {
        let mut dest = if self.current == 1 {
            self.max
        } else {
            self.current - 1
        };
        let pick1 = self.cups[self.current];
        let pick2 = self.cups[pick1];
        let pick3 = self.cups[pick2];
        self.cups[self.current] = self.cups[pick3];

        while [pick1, pick2, pick3].contains(&dest) {
            dest = if dest == 1 { self.max } else { dest - 1 };
        }

        let dest_next = self.cups[dest];
        self.cups[dest] = pick1;
        self.cups[pick3] = dest_next;

        self.current = self.cups[self.current];
    }

    fn to_vec(&self) -> Vec<usize> {
        let mut cups = vec![self.current];
        let mut last = self.current;
        for _ in 0..self.cups.len() - 2 {
            last = self.cups[last];
            cups.push(last);
        }
        cups
    }

    fn to_int(&self) -> usize {
        let cups = self.to_vec();
        let pre = cups.iter().take_while(|cup| **cup != 1);
        let post = cups.iter().skip_while(|cup| **cup != 1).skip(1);
        post.chain(pre).fold(0, |acc, cup| acc * 10 + *cup)
    }

    fn after(&self, find: usize, n: usize) -> Vec<usize> {
        self.to_vec()
            .iter()
            .skip_while(|cup| **cup != find)
            .skip(1)
            .take(n)
            .cloned()
            .collect()
    }
}

fn solve(cups: &[usize], extra: Option<usize>, moves: usize) -> usize {
    let mut game = Game::new(cups, extra);
    for _ in 0..moves {
        game.play()
    }
    if extra.is_none() {
        game.to_int()
    } else {
        game.after(1, 2).iter().product()
    }
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p23.txt");
    let cups = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(&cups, None, 100);
    let out2 = solve(&cups, Some(1000000), 10000000);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(solve(&cups, None, 10), 92658374);
        assert_eq!(solve(&cups, None, 100), 67384529);
    }

    #[test]
    fn test02() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(solve(&cups, Some(1000000), 10000000), 149245887792);
    }
}
