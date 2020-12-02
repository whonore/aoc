#[derive(PartialEq, Eq, Debug)]
struct Policy {
    char: String,
    min: usize,
    max: usize,
}

impl From<&str> for Policy {
    fn from(policy: &str) -> Self {
        let fields: Vec<&str> = policy.split(' ').collect();
        let range: Vec<&str> = fields[0].split('-').collect();
        Self {
            char: fields[1].into(),
            min: range[0].parse().unwrap(),
            max: range[1].parse().unwrap(),
        }
    }
}

impl Policy {
    fn is_valid(&self, pass: &str) -> bool {
        let n = pass.matches(&self.char).count();
        self.min <= n && n <= self.max
    }
}

fn solve(xs: &[(Policy, String)]) -> usize {
    xs.iter()
        .filter(|(policy, pass)| policy.is_valid(pass))
        .count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p02.txt");
    let xs: Vec<(Policy, String)> = input
        .lines()
        .map(|x| {
            let x: Vec<&str> = x.split(':').collect();
            (x[0].into(), x[1].into())
        })
        .collect();
    let out1 = solve(&xs);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy() {
        assert_eq!(
            Policy::from("1-3 a"),
            Policy {
                char: "a".into(),
                min: 1,
                max: 3
            }
        );
    }

    #[test]
    fn test01() {
        assert_eq!(
            solve(&[
                (
                    Policy {
                        char: "a".into(),
                        min: 1,
                        max: 3
                    },
                    "abcde".into()
                ),
                (
                    Policy {
                        char: "b".into(),
                        min: 1,
                        max: 3
                    },
                    "cdefg".into()
                ),
                (
                    Policy {
                        char: "c".into(),
                        min: 2,
                        max: 9
                    },
                    "ccccccccc".into()
                )
            ]),
            2
        );
    }
}
