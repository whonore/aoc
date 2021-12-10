use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bracket {
    Paren,
    Square,
    Curly,
    Angle,
}

impl TryFrom<char> for Bracket {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' | ')' => Ok(Self::Paren),
            '[' | ']' => Ok(Self::Square),
            '{' | '}' => Ok(Self::Curly),
            '<' | '>' => Ok(Self::Angle),
            _ => Err(c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open(Bracket),
    Close(Bracket),
}

impl TryFrom<char> for Token {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::Open(Bracket::Paren)),
            '[' => Ok(Self::Open(Bracket::Square)),
            '{' => Ok(Self::Open(Bracket::Curly)),
            '<' => Ok(Self::Open(Bracket::Angle)),
            ')' => Ok(Self::Close(Bracket::Paren)),
            ']' => Ok(Self::Close(Bracket::Square)),
            '}' => Ok(Self::Close(Bracket::Curly)),
            '>' => Ok(Self::Close(Bracket::Angle)),
            _ => Err(c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Chunk {
    bracket: Bracket,
    inner: Vec<Self>,
}

#[derive(Debug, Clone)]
struct ChunkParser<'t> {
    tokens: &'t [Token],
    stack: Vec<Bracket>,
    chunks: Vec<Vec<Chunk>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChunkErr {
    InProgress,
    Corrupted(Bracket, Bracket),
    Incomplete(Token),
}

impl<'a> ChunkParser<'a> {
    const fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            stack: vec![],
            chunks: vec![],
        }
    }
}

impl<'t> Iterator for ChunkParser<'t> {
    type Item = Result<Chunk, ChunkErr>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&token, rest)) = self.tokens.split_first() {
            self.tokens = rest;
            match token {
                Token::Open(b) => {
                    self.stack.push(b);
                    self.chunks.push(vec![]);
                    Some(Err(ChunkErr::InProgress))
                }
                Token::Close(close) => match self.stack.pop() {
                    Some(open) => {
                        if open == close {
                            let chunk = Chunk {
                                bracket: open,
                                inner: self.chunks.pop().unwrap_or_else(Vec::new),
                            };
                            if self.stack.is_empty() {
                                self.chunks.clear();
                                Some(Ok(chunk))
                            } else {
                                if let Some(chunks) = self.chunks.last_mut() {
                                    chunks.push(chunk);
                                } else {
                                    self.chunks.push(vec![chunk]);
                                }
                                Some(Err(ChunkErr::InProgress))
                            }
                        } else {
                            Some(Err(ChunkErr::Corrupted(open, close)))
                        }
                    }
                    None => Some(Err(ChunkErr::Incomplete(token))),
                },
            }
        } else {
            let last = self.stack.pop()?;
            Some(Err(ChunkErr::Incomplete(Token::Open(last))))
        }
    }
}

fn part1(nav: &[Vec<Token>]) -> u64 {
    nav.iter()
        .map(|tokens| {
            let mut parser = ChunkParser::new(tokens);
            parser
                .find_map(|chunk| match chunk {
                    Err(ChunkErr::Corrupted(_, found)) => Some(match found {
                        Bracket::Paren => 3,
                        Bracket::Square => 57,
                        Bracket::Curly => 1197,
                        Bracket::Angle => 25137,
                    }),
                    _ => None,
                })
                .unwrap_or(0)
        })
        .sum()
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d10.txt");
    let nav = input
        .lines()
        .map(|line| {
            line.chars()
                .map(Token::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|c| format!("Invalid char: {}", c))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&nav);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! chunk {
        ($b:expr,$cs:expr) => {{
            Chunk {
                bracket: Bracket::try_from($b).unwrap(),
                inner: $cs,
            }
        }};
    }

    fn parse(line: &str) -> Vec<Result<Chunk, ChunkErr>> {
        let tokens = line
            .chars()
            .map(|c| Token::try_from(c).unwrap())
            .collect::<Vec<_>>();
        ChunkParser::new(&tokens)
            .filter(|chunk| !matches!(chunk, Err(ChunkErr::InProgress)))
            .collect()
    }

    #[test]
    fn test_parse_ok() {
        assert_eq!(
            parse("([])"),
            vec![Ok(chunk!['(', vec![chunk!['[', vec![]]]])]
        );
        assert_eq!(
            parse("{()()()}"),
            vec![Ok(chunk![
                '{',
                vec![
                    chunk!['(', vec![]],
                    chunk!['(', vec![]],
                    chunk!['(', vec![]]
                ]
            ])]
        );
        assert_eq!(
            parse("[<>({}){}[([])<>]]"),
            vec![Ok(chunk![
                '[',
                vec![
                    chunk!['<', vec![]],
                    chunk!['(', vec![chunk!['{', vec![]],]],
                    chunk!['{', vec![]],
                    chunk![
                        '[',
                        vec![chunk!['(', vec![chunk!['[', vec![]],]], chunk!['<', vec![]],]
                    ],
                ]
            ])]
        );
    }

    #[test]
    fn test_parse_corrupted() {
        assert_eq!(
            parse("(]"),
            vec![Err(ChunkErr::Corrupted(Bracket::Paren, Bracket::Square))]
        );
        assert_eq!(
            parse("{()()()>"),
            vec![Err(ChunkErr::Corrupted(Bracket::Curly, Bracket::Angle))]
        );
        assert_eq!(
            parse("<([]){()}[{}])"),
            vec![Err(ChunkErr::Corrupted(Bracket::Angle, Bracket::Paren))]
        );
    }

    #[test]
    fn test01() {
        let nav = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Token::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        assert_eq!(part1(&nav), 26397);
    }
}
