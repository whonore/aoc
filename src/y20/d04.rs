use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[allow(clippy::upper_case_acronyms)]
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

const REQUIRED: [Field; 7] = [BYR, IYR, EYR, HGT, HCL, ECL, PID];

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
            _ => Err(format!("Invalid field: {}", f)),
        }
    }
}

fn in_range(n: &str, min: u32, max: u32) -> bool {
    n.parse::<u32>()
        .map(|v| min <= v && v <= max)
        .unwrap_or(false)
}

impl Field {
    fn validate(&self, val: &str) -> bool {
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        match self {
            BYR => in_range(val, 1920, 2002),
            IYR => in_range(val, 2010, 2020),
            EYR => in_range(val, 2020, 2030),
            HGT => HGT_RE.captures(val).map_or(false, |caps| match &caps[2] {
                "cm" => in_range(&caps[1], 150, 193),
                "in" => in_range(&caps[1], 59, 76),
                _ => false,
            }),
            HCL => HCL_RE.is_match(val),
            ECL => ECL_RE.is_match(val),
            PID => PID_RE.is_match(val),
            CID => true,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Passport {
    fields: HashMap<Field, String>,
}

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
    fn validate(&self, check_value: bool) -> bool {
        REQUIRED.iter().all(|f| self.fields.contains_key(f))
            && (!check_value || self.fields.iter().all(|(f, v)| f.validate(v)))
    }
}

fn solve(passports: &[Passport], check_value: bool) -> usize {
    passports.iter().filter(|p| p.validate(check_value)).count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d04.txt");
    let passports = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&passports, false);
    let out2 = solve(&passports, true);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(BYR.validate("2002"));
        assert!(!BYR.validate("2003"));
        assert!(HGT.validate("60in"));
        assert!(HGT.validate("190cm"));
        assert!(!HGT.validate("190in"));
        assert!(!HGT.validate("190"));
        assert!(HCL.validate("#123abc"));
        assert!(!HCL.validate("#123abz"));
        assert!(!HCL.validate("123abc"));
        assert!(ECL.validate("brn"));
        assert!(!ECL.validate("wat"));
        assert!(PID.validate("000000001"));
        assert!(!PID.validate("0123456789"));
    }

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
        assert_eq!(solve(&passports, false), 2)
    }

    #[test]
    fn test02() {
        let invalid: Vec<Passport> = [
            "eyr:1972 cid:100\n\
             hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019\n\
             hcl:#602927 eyr:1967 hgt:170cm\n\
             ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012\n\
             ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz\n\
             eyr:2038 hcl:74454a iyr:2023\n\
             pid:3556412378 byr:2007",
        ]
        .iter()
        .map(|p| p.parse().unwrap())
        .collect();

        let valid: Vec<Passport> = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
             hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989\n\
             iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785\n\
             hgt:164cm byr:2001 iyr:2015 cid:88\n\
             pid:545766238 ecl:hzl\n\
             eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .iter()
        .map(|p| p.parse().unwrap())
        .collect();

        assert_eq!(solve(&invalid, true), 0);
        assert_eq!(solve(&valid, true), valid.len());
    }
}
