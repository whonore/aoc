use std::collections::HashSet;
use std::str::FromStr;

enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}
use Instruction::*;

impl FromStr for Instruction {
    type Err = String;

    fn from_str(inst: &str) -> Result<Self, Self::Err> {
        let inst = inst.split_whitespace().collect::<Vec<_>>();
        let arg = inst[1]
            .parse::<i32>()
            .map_err(|_| format!("Invalid argument {}", inst[1]))?;
        match inst[0] {
            "nop" => Ok(Nop),
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            _ => Err(format!("Invalid instruction {}", inst[0])),
        }
    }
}

struct Program {
    instrs: Vec<Instruction>,
    acc: i32,
    iptr: usize,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(prog: &str) -> Result<Self, Self::Err> {
        let instrs = prog
            .lines()
            .map(|i| i.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Program {
            instrs,
            acc: 0,
            iptr: 0,
        })
    }
}

impl Program {
    fn reset(&mut self) {
        self.acc = 0;
        self.iptr = 0;
    }

    fn step(&mut self) {
        match &self.instrs[self.iptr] {
            Nop => {
                self.iptr += 1;
            }
            Acc(n) => {
                self.acc += n;
                self.iptr += 1;
            }
            Jmp(n) => {
                self.iptr = ((self.iptr as i32) + n) as usize;
            }
        }
    }
}

fn solve(prog: &mut Program) -> i32 {
    let mut visited = HashSet::<usize>::new();
    while !visited.contains(&prog.iptr) {
        visited.insert(prog.iptr);
        prog.step();
    }
    prog.acc
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p08.txt");
    let mut prog = input.parse::<Program>()?;
    let out1 = solve(&mut prog);
    prog.reset();
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut prog = "nop +0\n\
                    acc +1\n\
                    jmp +4\n\
                    acc +3\n\
                    jmp -3\n\
                    acc -99\n\
                    acc +1\n\
                    jmp -4\n\
                    acc +6"
            .parse::<Program>()
            .unwrap();
        assert_eq!(solve(&mut prog), 5);
    }
}
