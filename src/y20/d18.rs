use std::collections::VecDeque;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Plus,
    Mult,
}

#[derive(Debug, Copy, Clone)]
enum Token {
    LParen,
    RParen,
    Op(Op),
    Number(i64),
}
use Token::*;

impl FromStr for Token {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        match e.chars().next().unwrap() {
            '(' => Ok(LParen),
            ')' => Ok(RParen),
            '+' => Ok(Op(Op::Plus)),
            '*' => Ok(Op(Op::Mult)),
            '0'..='9' => Ok(Number(e.parse::<i64>().unwrap())),
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
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { toks, depth: 0 })
    }
}

#[derive(Debug)]
enum Expr {
    BinOp(Op, Box<Expr>, Box<Expr>),
    Scalar(i64),
}
use Expr::*;

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            BinOp(op, lhs, rhs) => {
                let lhs = lhs.eval();
                let rhs = rhs.eval();
                match op {
                    Op::Plus => lhs + rhs,
                    Op::Mult => lhs * rhs,
                }
            }
            Scalar(v) => *v,
        }
    }
}

struct SamePrec(Expr);

impl FromStr for SamePrec {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        let toks = e.parse::<Lexer>()?;
        Self::try_from(toks)
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
impl TryFrom<Lexer> for SamePrec {
    type Error = String;

    fn try_from(mut lex: Lexer) -> Result<Self, Self::Error> {
        Ok(Self(Self::parse_expr(&mut lex)?))
    }
}

impl SamePrec {
    fn parse_expr(lex: &mut Lexer) -> Result<Expr, String> {
        let lhs = Self::parse_term(lex)?;
        Self::parse_expr2(lhs, lex)
    }

    fn parse_expr2(lhs: Expr, lex: &mut Lexer) -> Result<Expr, String> {
        match lex.toks.front().copied() {
            Some(Op(op)) => {
                lex.toks.pop_front();
                let rhs = Self::parse_term(lex)?;
                let lhs = BinOp(op, Box::new(lhs), Box::new(rhs));
                Self::parse_expr2(lhs, lex)
            }
            _ => Ok(lhs),
        }
    }

    fn parse_term(lex: &mut Lexer) -> Result<Expr, String> {
        match lex.toks.pop_front() {
            Some(Number(v)) => Ok(Scalar(v)),
            Some(LParen) => {
                lex.depth += 1;
                let e = Self::parse_expr(lex)?;
                match lex.toks.pop_front() {
                    Some(RParen) => {
                        lex.depth -= 1;
                        Ok(e)
                    }
                    _ => Err("Found unmatched (".into()),
                }
            }
            Some(tok) => Err(format!("Expected term, found {:?}", tok)),
            None => Err("Expected term, found EOF".into()),
        }
    }
}

struct DiffPrec(Expr);

impl FromStr for DiffPrec {
    type Err = String;

    fn from_str(e: &str) -> Result<Self, Self::Err> {
        let toks = e.parse::<Lexer>()?;
        Self::try_from(toks)
    }
}

/*
 * expr -> expr * term | term
 * term -> term + factor | factor
 * factor -> INT | ( expr )
 *
 * expr -> term expr'
 * expr' -> * term expr' | ϵ
 * term -> factor term'
 * term' -> + factor term' | ϵ
 * factor -> INT | ( expr )
 */
impl TryFrom<Lexer> for DiffPrec {
    type Error = String;

    fn try_from(mut lex: Lexer) -> Result<Self, Self::Error> {
        Ok(Self(Self::parse_expr(&mut lex)?))
    }
}

impl DiffPrec {
    fn parse_expr(lex: &mut Lexer) -> Result<Expr, String> {
        let lhs = Self::parse_term(lex)?;
        Self::parse_expr2(lhs, lex)
    }

    fn parse_expr2(lhs: Expr, lex: &mut Lexer) -> Result<Expr, String> {
        match lex.toks.front() {
            Some(Op(Op::Mult)) => {
                lex.toks.pop_front();
                let rhs = Self::parse_term(lex)?;
                let lhs = BinOp(Op::Mult, Box::new(lhs), Box::new(rhs));
                Self::parse_expr2(lhs, lex)
            }
            _ => Ok(lhs),
        }
    }

    fn parse_term(lex: &mut Lexer) -> Result<Expr, String> {
        let lhs = Self::parse_factor(lex)?;
        Self::parse_term2(lhs, lex)
    }

    fn parse_term2(lhs: Expr, lex: &mut Lexer) -> Result<Expr, String> {
        match lex.toks.front() {
            Some(Op(Op::Plus)) => {
                lex.toks.pop_front();
                let rhs = Self::parse_factor(lex)?;
                let lhs = BinOp(Op::Plus, Box::new(lhs), Box::new(rhs));
                Self::parse_term2(lhs, lex)
            }
            _ => Ok(lhs),
        }
    }

    fn parse_factor(lex: &mut Lexer) -> Result<Expr, String> {
        match lex.toks.pop_front() {
            Some(Number(v)) => Ok(Scalar(v)),
            Some(LParen) => {
                lex.depth += 1;
                let e = Self::parse_expr(lex)?;
                match lex.toks.pop_front() {
                    Some(RParen) => {
                        lex.depth -= 1;
                        Ok(e)
                    }
                    _ => Err("Found unmatched (".into()),
                }
            }
            Some(tok) => Err(format!("Expected factor, found {:?}", tok)),
            None => Err("Expected term, found EOF".into()),
        }
    }
}

fn solve(exps: &[Expr]) -> i64 {
    exps.iter().map(Expr::eval).sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d18.txt");
    let exps = input
        .lines()
        .map(|x| x.parse::<SamePrec>().map(|e| e.0))
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = solve(&exps);
    let exps = input
        .lines()
        .map(|x| x.parse::<DiffPrec>().map(|e| e.0))
        .collect::<Result<Vec<_>, _>>()?;
    let out2 = solve(&exps);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!("2".parse::<SamePrec>().unwrap().0.eval(), 2);
        assert_eq!("(2)".parse::<SamePrec>().unwrap().0.eval(), 2);
        assert_eq!(
            "1 + 2 * 3 + 4 * 5 + 6"
                .parse::<SamePrec>()
                .unwrap()
                .0
                .eval(),
            71
        );
        assert_eq!(
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<SamePrec>()
                .unwrap()
                .0
                .eval(),
            51
        );
        assert_eq!("2 * 3 + (4 * 5)".parse::<SamePrec>().unwrap().0.eval(), 26);
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<SamePrec>()
                .unwrap()
                .0
                .eval(),
            437
        );
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<SamePrec>()
                .unwrap()
                .0
                .eval(),
            12240
        );
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<SamePrec>()
                .unwrap()
                .0
                .eval(),
            13632
        );
    }

    #[test]
    fn test02() {
        assert_eq!("2".parse::<DiffPrec>().unwrap().0.eval(), 2);
        assert_eq!("(2)".parse::<DiffPrec>().unwrap().0.eval(), 2);
        assert_eq!(
            "1 + 2 * 3 + 4 * 5 + 6"
                .parse::<DiffPrec>()
                .unwrap()
                .0
                .eval(),
            231
        );
        assert_eq!(
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<DiffPrec>()
                .unwrap()
                .0
                .eval(),
            51
        );
        assert_eq!("2 * 3 + (4 * 5)".parse::<DiffPrec>().unwrap().0.eval(), 46);
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<DiffPrec>()
                .unwrap()
                .0
                .eval(),
            1445
        );
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<DiffPrec>()
                .unwrap()
                .0
                .eval(),
            669_060
        );
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<DiffPrec>()
                .unwrap()
                .0
                .eval(),
            23340
        );
    }
}
