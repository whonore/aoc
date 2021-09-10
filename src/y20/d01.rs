fn find_sum(n: u32, tgt: u32, xs: &[u32]) -> Option<Vec<u32>> {
    if n == 1 {
        xs.contains(&tgt).then(|| vec![tgt])
    } else {
        let (x, mut ys) = xs.iter().enumerate().find_map(|(i, x)| {
            (*x < tgt)
                .then(|| find_sum(n - 1, tgt - x, &xs[i + 1..]).map(|sum| (x, sum)))
                .flatten()
        })?;
        ys.push(*x);
        Some(ys)
    }
}

fn solve(n: u32, xs: &[u32]) -> Option<u32> {
    find_sum(n, 2020, xs).map(|xs| xs.iter().product())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d01.txt");
    let xs: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    let out1 = solve(2, &xs).ok_or("No solution found")?;
    let out2 = solve(3, &xs).ok_or("No solution found")?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(2, &[1721, 979, 366, 299, 675, 1456]), Some(514_579));
    }

    #[test]
    fn test02() {
        assert_eq!(
            solve(3, &[1721, 979, 366, 299, 675, 1456]),
            Some(241_861_950)
        );
    }
}
