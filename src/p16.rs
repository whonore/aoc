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

struct Fields(HashMap<String, Vec<Range<u64>>>);

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
    fn valid(&self, val: u64) -> bool {
        self.0
            .values()
            .any(|ranges| ranges.iter().any(|r| r.contains(val)))
    }

    fn valid_at(&self, field: &str, vals: &[u64]) -> bool {
        vals.iter()
            .all(|v| self.0[field].iter().any(|r| r.contains(*v)))
    }
}

struct Ticket(Vec<u64>);

impl FromStr for Ticket {
    type Err = String;

    fn from_str(tick: &str) -> Result<Self, Self::Err> {
        Ok(Ticket(
            tick.split(',').map(|x| x.parse::<u64>().unwrap()).collect(),
        ))
    }
}

impl Ticket {
    fn invalid(&self, fields: &Fields) -> Vec<u64> {
        self.0
            .iter()
            .filter(|v| !fields.valid(**v))
            .copied()
            .collect()
    }

    fn valid(&self, fields: &Fields) -> bool {
        self.invalid(fields).is_empty()
    }
}

enum Mode {
    ErrorRate,
    IdentifyFields,
}
use Mode::*;

fn error_rate(fields: &Fields, ticks: &[Ticket]) -> u64 {
    ticks.iter().flat_map(|tick| tick.invalid(fields)).sum()
}

fn identify_fields(fields: &Fields, ticks: &[Ticket]) -> Vec<String> {
    let ticks = ticks
        .iter()
        .filter(|tick| tick.valid(&fields))
        .collect::<Vec<_>>();
    let mut tick_fields = (0..ticks[0].0.len())
        .map(|idx| {
            let vs = ticks.iter().map(|tick| tick.0[idx]).collect::<Vec<_>>();
            fields
                .0
                .keys()
                .filter(|f| fields.valid_at(f, &vs))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut done = tick_fields
        .iter()
        .filter(|fs| fs.len() == 1)
        .map(|fs| fs[0])
        .collect::<Vec<_>>();

    while done.len() != fields.0.keys().len() {
        for fs in tick_fields.iter_mut() {
            if 1 < fs.len() {
                fs.retain(|f| !done.contains(f));
                if fs.len() == 1 {
                    done.push(fs[0])
                }
            }
        }
    }

    tick_fields.iter().map(|fs| fs[0]).cloned().collect()
}

fn solve(fields: &Fields, mytick: &Ticket, ticks: &[Ticket], mode: Mode) -> u64 {
    match mode {
        ErrorRate => error_rate(fields, ticks),
        IdentifyFields => identify_fields(fields, ticks)
            .iter()
            .enumerate()
            .filter(|(_, field)| field.starts_with("departure"))
            .map(|(idx, _)| mytick.0[idx])
            .product(),
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
    let out2 = solve(&fields, &mytick, &ticks, IdentifyFields);
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

    #[test]
    fn test02() {
        let fields = Fields(
            [
                ("class".to_string(), vec![Range(0, 1), Range(4, 19)]),
                ("row".to_string(), vec![Range(0, 5), Range(8, 19)]),
                ("seat".to_string(), vec![Range(0, 13), Range(16, 19)]),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        let ticks = vec![
            Ticket(vec![3, 9, 18]),
            Ticket(vec![15, 1, 5]),
            Ticket(vec![5, 14, 9]),
        ];
        assert_eq!(
            identify_fields(&fields, &ticks),
            vec!["row", "class", "seat"]
        )
    }
}
