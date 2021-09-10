// q = gcd(x, y) = x * x0 + y * y0
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn euclid(x: u64, y: u64) -> (u64, i64, i64) {
    let mut x = x as i64;
    let mut y = y as i64;
    let mut x0 = 1;
    let mut x1 = 0;
    let mut y0 = 0;
    let mut y1 = 1;
    let mut q = 0;
    while y.is_positive() {
        q = x / y;
        let (x_new, y_new) = (y, x % y);
        let (x0_new, x1_new) = (x1, x0 - q * x1);
        let (y0_new, y1_new) = (y1, y0 - q * y1);
        x = x_new;
        y = y_new;
        x0 = x0_new;
        x1 = x1_new;
        y0 = y0_new;
        y1 = y1_new;
    }
    assert!(0 <= q);
    (q as u64, x0, y0)
}

// a * x mod n = 1
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn invmod(a: u64, n: u64) -> u64 {
    let (_, mut x, _) = euclid(a, n);
    while x.is_negative() {
        x += n as i64;
    }
    (x as u64) % n
}

fn solve(busses: &[Option<u64>], start: Option<u64>) -> Result<u64, String> {
    if let Some(start) = start {
        let (bus, wait) = busses
            .iter()
            .filter_map(Option::as_ref)
            .map(|bus| (bus, bus * (1 + (start / bus)) - start))
            .min_by_key(|(_, wait)| *wait)
            .ok_or("No bus found")?;
        Ok(bus * wait)
    } else {
        // find t s.t., forall i, t mod mods[i].1 = mods[i].0
        let mods = busses
            .iter()
            .enumerate()
            .filter_map(|(idx, bus)| bus.map(|bus| ((bus - (idx as u64) % bus) % bus, bus)))
            .collect::<Vec<(u64, u64)>>();
        let mod_prod = mods.iter().map(|(_, modu)| modu).product::<u64>();
        // Chinese remainder theorem
        // https://shainer.github.io/crypto/math/2017/10/22/chinese-remainder-theorem.html
        Ok(mods
            .iter()
            .map(|(rem, modu)| rem * (mod_prod / modu) * invmod(mod_prod / modu, *modu))
            .sum::<u64>()
            % mod_prod)
    }
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d13.txt");
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse::<u64>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|bus| bus.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let out1 = solve(&busses, Some(start))?;
    let out2 = solve(&busses, None)?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let busses = [
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19),
        ];
        assert_eq!(solve(&busses, Some(939)), Ok(295));
    }

    #[allow(clippy::shadow_unrelated)]
    #[test]
    fn test02() {
        let busses = [Some(17), None, Some(13), Some(19)];
        assert_eq!(solve(&busses, None), Ok(3417));
        let busses = [Some(67), Some(7), Some(59), Some(61)];
        assert_eq!(solve(&busses, None), Ok(754_018));
        let busses = [Some(67), None, Some(7), Some(59), Some(61)];
        assert_eq!(solve(&busses, None), Ok(779_210));
        let busses = [Some(67), Some(7), None, Some(59), Some(61)];
        assert_eq!(solve(&busses, None), Ok(1_261_476));
        let busses = [Some(1789), Some(37), Some(47), Some(1889)];
        assert_eq!(solve(&busses, None), Ok(1_202_161_486));
    }
}
