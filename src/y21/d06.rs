use std::convert::TryInto;

type Timer = usize;

const MAX_TIMER: Timer = 6;
const NEW_TIMER: Timer = 8;

#[derive(Debug, Clone)]
struct Fish([Timer; NEW_TIMER + 2]);

impl Fish {
    fn new(timers: &[Timer]) -> Self {
        Self(
            (0..=NEW_TIMER + 1)
                .map(|n| timers.iter().filter(|&&t| t == n).count())
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
        )
    }

    fn run_days(&mut self, days: u64) {
        for _ in 0..days {
            self.0[NEW_TIMER + 1] += self.0[0];
            self.decrement();
        }
    }

    fn decrement(&mut self) {
        let mut timers = [0; NEW_TIMER + 2];
        timers[MAX_TIMER] += self.0[0];
        for (n, t) in self.0[1..].iter().enumerate() {
            timers[n] += t;
        }
        self.0 = timers;
    }

    fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

fn part1(mut fish: Fish, days: u64) -> usize {
    fish.run_days(days);
    fish.count()
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d06.txt");
    let fish = Fish::new(
        &input
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid timer")?,
    );
    let out1 = part1(fish.clone(), 80);
    let out2 = part1(fish, 256);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let fish = Fish::new(&[3, 4, 3, 1, 2]);
        assert_eq!(part1(fish.clone(), 18), 26);
        assert_eq!(part1(fish, 80), 5934);
    }

    #[test]
    fn test02() {
        let fish = Fish::new(&[3, 4, 3, 1, 2]);
        assert_eq!(part1(fish, 256), 26_984_457_539);
    }
}
