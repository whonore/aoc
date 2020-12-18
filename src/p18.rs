use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Plus,
    Mult,
}
use Op::*;

#[derive(Debug, Copy, Clone)]
enum Token {
    TLParen,
    TRParen,
    TOp(Op),
    TNumber(i64),
}
use Token::*;

impl FromStr for Token {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        match e.chars().next().unwrap() {
            '(' => Ok(TLParen),
            ')' => Ok(TRParen),
            '+' => Ok(TOp(Plus)),
            '*' => Ok(TOp(Mult)),
            '0'..='9' => Ok(TNumber(e.parse::<i64>().unwrap())),
            _ => Err(format!("Invalid token {}", e)),
        }
    }
}

#[derive(Debug)]
struct Lexer {
    toks: Vec<Token>,
    depth: u32,
}

impl FromStr for Lexer {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        let toks = e
            .replace('(', " ( ")
            .replace(')', " ) ")
            .split_whitespace()
            .map(|tok| tok.parse::<Token>())
            .collect::<Result<_, _>>()?;
        Ok(Lexer { toks, depth: 0 })
    }
}

impl Lexer {
    fn reverse(&mut self) {
        self.toks = self
            .toks
            .iter()
            .rev()
            .map(|tok| match tok {
                TLParen => TRParen,
                TRParen => TLParen,
                tok => *tok,
            })
            .collect()
    }
}

#[derive(Debug)]
enum Expr {
    BinOp(Op, Box<Expr>, Box<Expr>),
    Scalar(i64),
}
use Expr::*;

impl FromStr for Expr {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        let mut lex = e.parse::<Lexer>()?;
        lex.reverse();
        Expr::try_from(&mut lex)
    }
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            BinOp(op, lhs, rhs) => {
                let lhs = lhs.eval();
                let rhs = rhs.eval();
                match op {
                    Plus => lhs + rhs,
                    Mult => lhs * rhs,
                }
            }
            Scalar(v) => *v,
        }
    }
}

fn pop_left<A>(xs: &mut Vec<A>) -> Option<A> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.remove(0))
    }
}

impl TryFrom<&mut Lexer> for Expr {
    type Error = String;

    fn try_from(mut lex: &mut Lexer) -> Result<Self, Self::Error> {
        let lhs = match pop_left(&mut lex.toks) {
            Some(TLParen) => {
                lex.depth += 1;
                // NOTE: Needed to satisfy the borrow checker
                let lex = &mut *lex;
                Expr::try_from(lex)
            }
            Some(TNumber(v)) => Ok(Scalar(v)),
            Some(tok) => Err(format!("Expected term, found {:?}", tok)),
            _ => Err("Expected term, found EOF".into()),
        }?;
        if let Some(op) = pop_left(&mut lex.toks) {
            let op = match op {
                TOp(op) => Ok(op),
                TRParen => {
                    if 0 < lex.depth {
                        lex.depth -= 1;
                        return Ok(lhs);
                    } else {
                        Err("Found unmatched )".into())
                    }
                }
                tok => Err(format!("Expected op, found {:?}", tok)),
            }?;
            let rhs = Expr::try_from(lex)?;
            Ok(BinOp(op, Box::new(lhs), Box::new(rhs)))
        } else {
            Ok(lhs)
        }
    }
}

fn solve(exps: &[Expr]) -> i64 {
    exps.iter().map(|e| e.eval()).sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p18.txt");
    let exps = input
        .lines()
        .map(|x| x.parse::<Expr>())
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&exps);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!("2".parse::<Expr>().unwrap().eval(), 2);
        assert_eq!("(2)".parse::<Expr>().unwrap().eval(), 2);
        assert_eq!("2 * 3 + (4 * 5)".parse::<Expr>().unwrap().eval(), 26);
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            437
        );
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            12240
        );
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<Expr>()
                .unwrap()
                .eval(),
            13632
        );
    }
}
