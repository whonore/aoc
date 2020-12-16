use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct Range<A>(A, A);

impl<A: PartialOrd> Range<A> {
    fn contains(&self, x: A) -> bool {
        self.0 <= x && x <= self.1
    }
}

impl<A: FromStr> FromStr for Range<A> {
    type Err = String;

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        let mut range = range
            .split('-')
            .map(|x| x.parse::<A>().map_err(|_| "Invalid range"));
        let min = range.next().unwrap()?;
        let max = range.next().unwrap()?;
        Ok(Range(min, max))
    }
}

struct Fields(HashMap<String, Vec<Range<u32>>>);

impl FromStr for Fields {
    type Err = String;

    fn from_str(fields: &str) -> Result<Self, Self::Err> {
        let fields = fields
            .lines()
            .map(|field| {
                let mut field = field.split(": ");
                let name = field.next().unwrap();
                let ranges = field
                    .next()
                    .unwrap()
                    .split(" or ")
                    .map(|r| r.parse::<Range<_>>())
                    .collect::<Result<_, _>>()?;
                Ok((name.to_string(), ranges))
            })
            .collect::<Result<_, String>>()?;
        Ok(Fields(fields))
    }
}

impl Fields {
    fn valid(&self, val: u32) -> bool {
        self.0
            .values()
            .any(|ranges| ranges.iter().any(|r| r.contains(val)))
    }
}

struct Ticket(Vec<u32>);

impl FromStr for Ticket {
    type Err = String;

    fn from_str(tick: &str) -> Result<Self, Self::Err> {
        Ok(Ticket(
            tick.split(',').map(|x| x.parse::<u32>().unwrap()).collect(),
        ))
    }
}

impl Ticket {
    fn invalid(&self, fields: &Fields) -> Vec<u32> {
        self.0
            .iter()
            .filter(|v| !fields.valid(**v))
            .copied()
            .collect()
    }
}

enum Mode {
    ErrorRate,
}
use Mode::*;

fn error_rate(fields: &Fields, ticks: &[Ticket]) -> u32 {
    ticks.iter().flat_map(|tick| tick.invalid(fields)).sum()
}

fn solve(fields: &Fields, mytick: &Ticket, ticks: &[Ticket], mode: Mode) -> u32 {
    match mode {
        ErrorRate => error_rate(fields, ticks),
    }
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p16.txt");
    let mut sections = input.split("\n\n");
    let fields = sections.next().unwrap().parse::<Fields>()?;
    let mytick = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .parse::<Ticket>()?;
    let ticks = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.parse::<Ticket>())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&fields, &mytick, &ticks, ErrorRate);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let fields = Fields(
            [
                ("class".to_string(), vec![Range(1, 3), Range(5, 7)]),
                ("row".to_string(), vec![Range(6, 11), Range(33, 44)]),
                ("seat".to_string(), vec![Range(13, 40), Range(45, 50)]),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        let mytick = Ticket(vec![7, 1, 14]);
        let ticks = vec![
            Ticket(vec![7, 3, 47]),
            Ticket(vec![40, 4, 50]),
            Ticket(vec![55, 2, 20]),
            Ticket(vec![38, 6, 12]),
        ];
        assert_eq!(solve(&fields, &mytick, &ticks, ErrorRate), 71);
    }
}
