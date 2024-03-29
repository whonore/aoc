use super::intcode::Intcode;

fn part1(prog: &Intcode) -> Result<i64, String> {
    let mut exec = prog.exec();
    exec.run_with(&[(1, 12), (2, 2)])?;
    Ok(exec[0])
}

fn part2(prog: &Intcode) -> Result<i64, String> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut exec = prog.exec();
            if exec.run_with(&[(1, noun), (2, verb)]).is_ok() && exec[0] == 19_690_720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No solution found".into())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d02.txt");
    let prog = input.parse()?;
    let out1 = part1(&prog)?;
    let out2 = part2(&prog)?;
    Ok(format!("{} {}", out1, out2))
}
