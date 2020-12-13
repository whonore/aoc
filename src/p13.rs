fn solve(busses: &[u32], start: u32) -> Result<u32, String> {
    let (bus, wait) = busses
        .iter()
        .map(|bus| (bus, bus * (1 + (start / bus)) - start))
        .min_by_key(|(_, wait)| *wait)
        .ok_or("No bus found")?;
    Ok(bus * wait)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p13.txt");
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse::<u32>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|bus| bus.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let out1 = solve(&busses, start)?;
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let busses = [7, 13, 59, 31, 19];
        assert_eq!(solve(&busses, 939), Ok(295));
    }
}
