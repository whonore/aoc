use std::env;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: aoc2020 {problem_number}".into());
    }

    let out = match args[1].parse::<u32>() {
        Ok(1) => p01::run(),
        Ok(2) => p02::run(),
        Ok(3) => p03::run(),
        Ok(4) => p04::run(),
        Ok(5) => p05::run(),
        Ok(6) => p06::run(),
        Ok(7) => p07::run(),
        Ok(8) => p08::run(),
        Ok(9) => p09::run(),
        Ok(10) => p10::run(),
        _ => Err("Invalid problem number".into()),
    }?;
    println!("{}", out);
    Ok(())
}
