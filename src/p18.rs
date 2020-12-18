use std::collections::VecDeque;
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
    toks: VecDeque<Token>,
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

#[derive(Debug)]
enum Expr {
    BinOp(Op, Box<Expr>, Box<Expr>),
    Scalar(i64),
}
use Expr::*;

impl FromStr for Expr {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        let toks = e.parse::<Lexer>()?;
        Expr::try_from(toks)
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

/*
 * expr -> expr + term | expr * term | term
 * term -> INT | ( expr )
 *
 * expr -> term expr'
 * expr' -> + term expr' | * term expr' | ϵ
 * term -> INT | ( expr )
 */
impl TryFrom<Lexer> for Expr {
    type Error = String;

    fn try_from(mut lex: Lexer) -> Result<Self, Self::Error> {
        parse_expr(&mut lex)
    }
}

fn parse_expr(lex: &mut Lexer) -> Result<Expr, String> {
    let lhs = parse_term(lex)?;
    parse_expr2(lhs, lex)
}

fn parse_expr2(lhs: Expr, lex: &mut Lexer) -> Result<Expr, String> {
    if let Some(op) = lex.toks.pop_front() {
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
        let rhs = parse_term(lex)?;
        let lhs = BinOp(op, Box::new(lhs), Box::new(rhs));
        parse_expr2(lhs, lex)
    } else {
        Ok(lhs)
    }
}

fn parse_term(lex: &mut Lexer) -> Result<Expr, String> {
    match lex.toks.pop_front() {
        Some(TNumber(v)) => Ok(Scalar(v)),
        Some(TLParen) => {
            lex.depth += 1;
            parse_expr(lex)
        }
        Some(tok) => Err(format!("Expected term, found {:?}", tok)),
        _ => Err("Expected term, found EOF".into()),
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
