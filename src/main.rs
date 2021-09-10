#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::use_self)]
#![warn(clippy::if_then_some_else_none)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::if_not_else)]
#![allow(clippy::pub_enum_variant_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]

use std::env;

macro_rules! days {
    ($($d:ident),*) => {
        [$(Box::new($d::run),)*]
    }
}

macro_rules! match_year {
    ($year:expr, $day:expr, $($y:ident),*) => {
        match $year {
            $($y::YEAR if 1 <= $day && $day <= $y::DAYS.len() => $y::DAYS[$day - 1](),)*
            _ => Err("Year or day out of range".into()),
        }
    }
}

mod y19;
mod y20;

type Run = dyn Fn() -> Result<String, String> + Sync;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(format!("Usage: {} YEAR DAY", args[0]));
    }
    let year = args[1].parse::<usize>().map_err(|_| "Invalid year")?;
    let day = args[2].parse::<usize>().map_err(|_| "Invalid day")?;

    println!("{}", match_year!(year, day, y19, y20)?);
    Ok(())
}
