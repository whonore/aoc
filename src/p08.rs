use std::collections::HashSet;
use std::str::FromStr;

enum Instruction {
    Nop(i32),
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
            "nop" => Ok(Nop(arg)),
            "acc" => Ok(Acc(arg)),
            "jmp" => Ok(Jmp(arg)),
            _ => Err(format!("Invalid instruction {}", inst[0])),
        }
    }
}

impl Instruction {
    const fn swap(&self) -> Option<Self> {
        match self {
            Nop(arg) => Some(Jmp(*arg)),
            Jmp(arg) => Some(Nop(*arg)),
            _ => None,
        }
    }
}

struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = String;

    fn from_str(instrs: &str) -> Result<Self, Self::Err> {
        let instrs = instrs
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(instrs))
    }
}

impl Instructions {
    fn prog(&self) -> Program {
        Program {
            instrs: &self.0,
            acc: 0,
            iptr: 0,
        }
    }

    fn swap(&mut self, idx: usize) -> bool {
        if let Some(inst) = self.0[idx].swap() {
            self.0[idx] = inst;
            true
        } else {
            false
        }
    }
}

struct Program<'a> {
    instrs: &'a [Instruction],
    acc: i32,
    iptr: usize,
}

enum Output {
    Terminate(i32),
    InfiniteLoop(i32),
}
use Output::*;

impl Program<'_> {
    fn step(&mut self) {
        match self.instrs[self.iptr] {
            Nop(_) => {
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

    fn run(&mut self) -> Output {
        let mut visited = HashSet::<usize>::new();
        loop {
            if visited.contains(&self.iptr) {
                return InfiniteLoop(self.acc);
            } else if self.instrs.len() <= self.iptr {
                return Terminate(self.acc);
            }
            visited.insert(self.iptr);
            self.step();
        }
    }
}

#[derive(Copy, Clone)]
enum Mode {
    DetectLoop,
    FixLoop,
}
use Mode::*;

fn solve(instrs: &mut Instructions, mode: Mode) -> Result<i32, String> {
    match mode {
        DetectLoop => match instrs.prog().run() {
            InfiniteLoop(acc) => Ok(acc),
            Terminate(_) => Err("No loop found".into()),
        },
        FixLoop => {
            for idx in 0..instrs.0.len() {
                match instrs.prog().run() {
                    InfiniteLoop(_) => {
                        if idx != 0 {
                            instrs.swap(idx - 1);
                        }
                        instrs.swap(idx);
                    }
                    Terminate(acc) => {
                        return Ok(acc);
                    }
                }
            }
            Err("No fix found".into())
        }
    }
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p08.txt");
    let mut instrs = input.parse::<Instructions>()?;
    let out1 = solve(&mut instrs, DetectLoop)?;
    let out2 = solve(&mut instrs, FixLoop)?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut instrs = "nop +0\n\
                          acc +1\n\
                          jmp +4\n\
                          acc +3\n\
                          jmp -3\n\
                          acc -99\n\
                          acc +1\n\
                          jmp -4\n\
                          acc +6"
            .parse::<Instructions>()
            .unwrap();
        assert_eq!(solve(&mut instrs, DetectLoop), Ok(5));
    }

    #[test]
    fn test02() {
        let mut instrs = "nop +0\n\
                          acc +1\n\
                          jmp +4\n\
                          acc +3\n\
                          jmp -3\n\
                          acc -99\n\
                          acc +1\n\
                          jmp -4\n\
                          acc +6"
            .parse::<Instructions>()
            .unwrap();
        assert_eq!(solve(&mut instrs, FixLoop), Ok(8));
    }
}
