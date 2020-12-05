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
        Ok(Ticket { row, col })
    }
}

impl Ticket {
    fn uid(&self) -> u16 {
        (self.row as u16) * 8 + (self.col as u16)
    }
}

fn solve(tickets: &[Ticket]) -> u16 {
    tickets.iter().map(|ticket| ticket.uid()).max().unwrap_or(0)
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p05.txt");
    let tickets = input
        .lines()
        .map(|ticket| ticket.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&tickets);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            solve(&[
                "BFFFBBFRRR".parse().unwrap(),
                "FFFBBBFRRR".parse().unwrap(),
                "BBFFBBFRLL".parse().unwrap(),
            ]),
            820
        )
    }
}
