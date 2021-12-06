type Timer = u64;

const MAX_TIMER: Timer = 6;
const NEW_TIMER: Timer = 8;

#[derive(Debug, Clone)]
struct Fish(Vec<Timer>);

impl Fish {
    fn run_days(&mut self, days: u64) {
        for _ in 0..days {
            self.0
                .extend([NEW_TIMER + 1].repeat(self.0.iter().filter(|&&t| t == 0).count()));
            self.decrement();
        }
    }

    fn decrement(&mut self) {
        self.0 = self
            .0
            .iter()
            .map(|&t| if t > 0 { t - 1 } else { MAX_TIMER })
            .collect();
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

fn part1(mut fish: Fish, days: u64) -> usize {
    fish.run_days(days);
    fish.count()
}

fn part2() -> usize {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d06.txt");
    let fish = Fish(
        input
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid timer")?,
    );
    let out1 = part1(fish, 80);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let fish = Fish(vec![3, 4, 3, 1, 2]);
        assert_eq!(part1(fish.clone(), 18), 26);
        assert_eq!(part1(fish, 80), 5934);
    }
}
