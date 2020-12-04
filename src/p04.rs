use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Hash)]
enum Field {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}
use Field::*;

impl FromStr for Field {
    type Err = String;

    fn from_str(f: &str) -> Result<Self, Self::Err> {
        match f {
            "byr" => Ok(BYR),
            "iyr" => Ok(IYR),
            "eyr" => Ok(EYR),
            "hgt" => Ok(HGT),
            "hcl" => Ok(HCL),
            "ecl" => Ok(ECL),
            "pid" => Ok(PID),
            "cid" => Ok(CID),
            _ => Err("Invalid field".into()),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Passport {
    fields: HashMap<Field, String>,
}
const REQUIRED: [Field; 7] = [BYR, IYR, EYR, HGT, HCL, ECL, PID];

impl FromStr for Passport {
    type Err = String;

    fn from_str(pass: &str) -> Result<Self, Self::Err> {
        let fields = pass
            .split_whitespace()
            .map(|kv| {
                let kv: Vec<_> = kv.split(':').collect();
                Ok((kv[0].parse()?, kv[1].into()))
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Self { fields })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        REQUIRED.iter().all(|f| self.fields.contains_key(f))
    }
}

fn solve(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p04.txt");
    let passports = input
        .split("\n\n")
        .map(|pass| pass.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&passports);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let passports: Vec<Passport> = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
             byr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
             hcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013\n\
             eyr:2024\n\
             ecl:brn pid:760753108 byr:1931\n\
             hgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648\n\
             iyr:2011 ecl:brn hgt:59in",
        ]
        .iter()
        .map(|p| p.parse().unwrap())
        .collect();
        assert_eq!(solve(&passports), 2)
    }
}
