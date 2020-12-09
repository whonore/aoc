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

    fn find_invalid(&mut self, data: &[u64]) -> Option<u64> {
        data.iter().find(|x| !self.process(**x)).copied()
    }
}

fn solve(data: &[u64], width: usize) -> Result<(u64, u64), String> {
    let (preamble, data) = data.split_at(width);
    let mut xmas = XMAS::new(preamble);
    let invalid = xmas
        .find_invalid(data)
        .ok_or_else(|| "No invalid data found".to_string())?;

    let data = [preamble, data].concat();
    let mut sum = 0;
    let mut start = 0;
    for end in 0..data.len() {
        if invalid == sum && 1 <= end - start {
            let min = data[start..end].iter().min().unwrap();
            let max = data[start..end].iter().max().unwrap();
            return Ok((invalid, min + max));
        }
        sum += data[end];
        while invalid < sum {
            sum -= data[start];
            start += 1;
        }
    }
    Err("No contiguous sum found".into())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p09.txt");
    let data = input
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (out1, out2) = solve(&data, 25)?;
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
        assert_eq!(solve(&data, 5), Ok((127, 62)));
    }
}
