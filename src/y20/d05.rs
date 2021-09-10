use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct Ticket {
    row: u8,
    col: u8,
}

impl FromStr for Ticket {
    type Err = String;

    fn from_str(ticket: &str) -> Result<Self, Self::Err> {
        let (row, col) = ticket.split_at(7);
        let row = u8::from_str_radix(&row.replace("F", "0").replace("B", "1"), 2)
            .map_err(|_| "Failed to parse row")?;
        let col = u8::from_str_radix(&col.replace("L", "0").replace("R", "1"), 2)
            .map_err(|_| "Failed to parse col")?;
        Ok(Self { row, col })
    }
}

impl Ticket {
    const fn uid(&self) -> u16 {
        (self.row as u16) * 8 + (self.col as u16)
    }
}

fn solve(tickets: &[Ticket], part1: bool) -> u16 {
    let uids = tickets.iter().map(Ticket::uid);
    if part1 {
        uids.max().unwrap_or(0)
    } else {
        let mut uids = uids.collect::<Vec<_>>();
        uids.sort();
        uids.windows(2)
            .find(|xs| xs[0] + 2 == xs[1])
            .map_or(0, |xs| xs[0] + 1)
    }
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d05.txt");
    let tickets = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&tickets, true);
    let out2 = solve(&tickets, false);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            solve(
                &[
                    "BFFFBBFRRR".parse().unwrap(),
                    "FFFBBBFRRR".parse().unwrap(),
                    "BBFFBBFRLL".parse().unwrap(),
                ],
                true
            ),
            820
        )
    }
}
