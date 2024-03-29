use super::intcode::Intcode;

fn part1(prog: &Intcode) -> Result<i64, String> {
    prog.exec()
        .read_vec(&[1])
        .write_to(vec![])
        .run()
        .and_then(|out| out.first().copied().ok_or_else(|| "No return value".into()))
}

fn part2(prog: &Intcode) -> Result<i64, String> {
    prog.exec()
        .read_vec(&[2])
        .write_to(vec![])
        .run()
        .and_then(|out| out.first().copied().ok_or_else(|| "No return value".into()))
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d09.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2(&prog)?;
    Ok(format!("{} {}", out1, out2))
}
