use std::env;

mod p01;
mod p02;
mod p03;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: aoc2020 {problem_number}".into());
    }

    let out = match args[1].parse::<u32>() {
        Ok(1) => p01::run(),
        Ok(2) => p02::run(),
        Ok(3) => p03::run(),
        _ => Err("Invalid problem number".into()),
    }?;
    println!("{}", out);
    Ok(())
}
