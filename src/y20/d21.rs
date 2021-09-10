use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

fn extract_singleton<A: Clone>(set: &HashSet<A>) -> Option<&A> {
    if set.len() == 1 {
        set.iter().next()
    } else {
        None
    }
}

fn counter<A, I>(xs: I) -> HashMap<A, usize>
where
    A: Clone + PartialEq + Eq + Hash,
    I: Iterator<Item = A>,
{
    let xs = xs.collect::<Vec<_>>();
    xs.iter()
        .cloned()
        .zip(xs.iter().map(|x| xs.iter().filter(|y| x == *y).count()))
        .collect()
}

type Ingredient = String;
type Allergen = String;

struct Foods {
    ingredients: HashMap<Ingredient, usize>,
    mapping: HashMap<Allergen, Ingredient>,
}

impl FromStr for Foods {
    type Err = String;

    fn from_str(foods: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            &foods
                .lines()
                .map(|line| {
                    let fields = line[..line.len() - 1]
                        .split(" (contains ")
                        .collect::<Vec<_>>();
                    (
                        fields[0].split_whitespace().collect::<Vec<_>>(),
                        fields[1].split(", ").collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        ))
    }
}

impl Foods {
    fn new(foods: &[(Vec<&str>, Vec<&str>)]) -> Self {
        let ingredients = counter(
            foods
                .iter()
                .flat_map(|(ings, _)| ings.iter().map(|ing| (*ing).to_string())),
        );
        let mut mapping = HashMap::<&str, HashSet<&str>>::new();
        for (ings, algs) in foods {
            let ings: HashSet<&str> = ings.iter().cloned().collect();
            for alg in algs {
                let ings_old = mapping.entry(alg).or_insert_with(|| ings.clone());
                *ings_old = ings_old.intersection(&ings).cloned().collect();
            }
        }

        let mut all_singles = HashSet::<&str>::new();
        loop {
            let singles = mapping
                .values()
                .filter_map(|ings| {
                    let ing = extract_singleton(ings)?;
                    if !all_singles.contains(ing) {
                        Some(ing)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect::<HashSet<_>>();
            if singles.is_empty() {
                break;
            }

            for algs in mapping.values_mut() {
                if 1 < algs.len() {
                    *algs = algs.difference(&singles).cloned().collect();
                }
            }
            all_singles = all_singles.union(&singles).cloned().collect();
        }
        let mapping = mapping
            .iter()
            .map(|(ing, algs)| {
                (
                    (*ing).to_string(),
                    (*extract_singleton(algs).unwrap()).to_string(),
                )
            })
            .collect::<HashMap<_, _>>();

        Self {
            ingredients,
            mapping,
        }
    }
}

fn solve(foods: &Foods) -> (usize, String) {
    let cnt = foods
        .ingredients
        .iter()
        .filter_map(|(ing, cnt)| {
            if foods.mapping.values().all(|ings| !ings.contains(ing)) {
                Some(cnt)
            } else {
                None
            }
        })
        .sum();
    let mut bad = foods.mapping.iter().collect::<Vec<_>>();
    bad.sort_by_key(|(alg, _)| *alg);
    (
        cnt,
        bad.drain(..)
            .map(|(_, ing)| ing)
            .cloned()
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/d21.txt");
    let foods = input.parse::<Foods>()?;
    let (out1, out2) = solve(&foods);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let foods = Foods::new(&[
            (
                vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                vec!["dairy", "fish"],
            ),
            (vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"], vec!["dairy"]),
            (vec!["sqjhc", "fvjkl"], vec!["soy"]),
            (vec!["sqjhc", "mxmxvkd", "sbzzf"], vec!["fish"]),
        ]);
        assert_eq!(solve(&foods), (5, "mxmxvkd,sqjhc,fvjkl".into()));
    }
}
