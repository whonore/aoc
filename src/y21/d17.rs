// TODO: Should be able to compute this from the x velocity and the target
// area.
const MAX_Y: i64 = 999;

type Target = (u64, u64, i64, i64);

const fn contains(x: u64, y: i64, tgt: Target) -> bool {
    let (xmin, xmax, ymin, ymax) = tgt;
    xmin <= x && x <= xmax && ymin <= y && y <= ymax
}

const fn overshot(x: u64, y: i64, tgt: Target) -> bool {
    let (_, xmax, ymin, _) = tgt;
    xmax < x || y < ymin
}

fn simulate(mut v_x: u64, mut v_y: i64, tgt: Target) -> (Vec<(u64, i64)>, bool) {
    let mut pos = vec![];
    let mut x = 0;
    let mut y = 0;
    while !contains(x, y, tgt) && !overshot(x, y, tgt) {
        pos.push((x, y));
        x += v_x;
        y += v_y;
        v_x = v_x.saturating_sub(1);
        v_y -= 1;
    }
    (pos, contains(x, y, tgt))
}

fn min_v_x(tgt: Target) -> u64 {
    #[allow(clippy::maybe_infinite_iter)]
    (0..).find(|n| n * (n + 1) >= 2 * tgt.0).unwrap()
}

const fn max_v_x(tgt: Target) -> u64 {
    tgt.1
}

const fn min_v_y(tgt: Target) -> i64 {
    tgt.2
}

const fn max_v_y(_v_x: u64, _tgt: Target) -> i64 {
    MAX_Y
}

fn part1(tgt: Target) -> i64 {
    (min_v_x(tgt)..=max_v_x(tgt))
        .flat_map(|x| {
            (min_v_y(tgt)..=max_v_y(x, tgt)).filter_map(move |y| {
                let (pos, ok) = simulate(x, y, tgt);
                ok.then(|| pos.iter().map(|xy| xy.1).max().unwrap())
            })
        })
        .max()
        .unwrap()
}

fn part2(tgt: Target) -> usize {
    (min_v_x(tgt)..=max_v_x(tgt))
        .flat_map(|x| (min_v_y(tgt)..=max_v_y(x, tgt)).filter(move |&y| simulate(x, y, tgt).1))
        .count()
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d17.txt");
    let tgt = {
        let mut fields = input["target area: ".len()..].trim().split(", ");
        let (minx, maxx) = {
            let mut xs = fields.next().unwrap()[2..]
                .split("..")
                .map(str::parse)
                .map(Result::unwrap);
            (xs.next().unwrap(), xs.next().unwrap())
        };
        let (miny, maxy) = {
            let mut ys = fields.next().unwrap()[2..]
                .split("..")
                .map(str::parse)
                .map(Result::unwrap);
            (ys.next().unwrap(), ys.next().unwrap())
        };
        (minx, maxx, miny, maxy)
    };
    let out1 = part1(tgt);
    let out2 = part2(tgt);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hits() {
        let tgt = (20, 30, -10, -5);
        assert!(simulate(7, 2, tgt).1);
        assert!(simulate(6, 3, tgt).1);
        assert!(simulate(9, 0, tgt).1);
        assert!(!simulate(17, -4, tgt).1);
    }

    #[test]
    fn test01() {
        assert_eq!(part1((20, 30, -10, -5)), 45);
    }

    #[test]
    fn test02() {
        assert_eq!(part2((20, 30, -10, -5)), 112);
    }
}
