use std::collections::HashSet;

fn find_sum(tgt: u32, xs: Vec<u32>) -> Option<(u32, u32)> {
    let xs: HashSet<u32> = xs.into_iter().collect();
    for x in &xs {
        if *x < tgt && xs.contains(&(tgt - x)) {
            return Some((*x, tgt - x));
        }
    }
    None
}

fn solve(xs: Vec<u32>) -> Option<u32> {
    find_sum(2020, xs).map(|(x, y)| x * y)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p01.txt");
    let xs = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    let out = solve(xs).ok_or("No solution found")?;
    Ok(out.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(solve(vec![1721, 979, 366, 299, 675, 1456]), Some(514579));
    }
}
