struct XMAS {
    data: Vec<u64>,
}

impl XMAS {
    fn new(preamble: &[u64]) -> Self {
        XMAS {
            data: preamble.into(),
        }
    }

    fn process(&mut self, x: u64) -> bool {
        let ok = self
            .data
            .iter()
            .filter_map(|y| if *y <= x { Some(x - y) } else { None })
            .any(|y| self.data.contains(&y));
        self.data.remove(0);
        self.data.push(x);
        ok
    }
}

fn solve(data: &[u64], width: usize) -> Result<u64, String> {
    let (preamble, data) = data.split_at(width);
    let mut xmas = XMAS::new(preamble);
    data.iter()
        .find(|x| !xmas.process(**x))
        .copied()
        .ok_or_else(|| "No invalid data found".into())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p09.txt");
    let data = input
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(&data, 25)?;
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let data = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(solve(&data, 5), Ok(127));
    }
}
