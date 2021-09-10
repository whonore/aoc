// sub ^ loop_sz mod 20201227
fn trans(sub: u64, loop_sz: u64) -> u64 {
    (0..loop_sz).fold(1, |acc, _| acc * sub % 2020_1227)
}

// 7 ^ x mod 20201227 = pub_key
const fn find_loop(pub_key: u64) -> u64 {
    let mut loop_sz = 0;
    let mut acc = 1;
    while acc != pub_key {
        acc = acc * 7 % 2020_1227;
        loop_sz += 1;
    }
    loop_sz
}

fn solve(card_pub: u64, door_pub: u64) -> u64 {
    let card_loop = find_loop(card_pub);
    let door_loop = find_loop(door_pub);
    assert_eq!(trans(card_pub, door_loop), trans(door_pub, card_loop));
    trans(card_pub, door_loop)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d25.txt");
    let pubs = input
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let out1 = solve(pubs[0], pubs[1]);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop() {
        assert_eq!(find_loop(5_764_801), 8);
        assert_eq!(trans(7, 8), 5_764_801);
        assert_eq!(find_loop(17_807_724), 11);
        assert_eq!(trans(7, 11), 17_807_724);
    }

    #[test]
    fn test01() {
        assert_eq!(solve(5_764_801, 17_807_724), 14_897_079);
    }
}
