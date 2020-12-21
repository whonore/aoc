use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

fn extract_singleton<A: Clone>(set: &HashSet<A>) -> Option<A> {
    if set.len() == 1 {
        set.iter().next().cloned()
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
    allergens: HashSet<Allergen>,
    mapping: HashMap<Allergen, HashSet<Ingredient>>,
}

impl FromStr for Foods {
    type Err = String;

    fn from_str(foods: &str) -> Result<Self, Self::Err> {
        Ok(Foods::new(
            &foods
                .lines()
                .map(|line| {
                    let fields = line[..line.len() - 1]
                        .split(" (contains ")
                        .collect::<Vec<_>>();
                    (
                        fields[0]
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                        fields[1]
                            .split(", ")
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        ))
    }
}

impl Foods {
    fn new(foods: &[(Vec<Ingredient>, Vec<Allergen>)]) -> Self {
        let ingredients: HashMap<Ingredient, usize> =
            counter(foods.iter().flat_map(|(i, _)| i).cloned());
        let allergens: HashSet<Allergen> = foods.iter().flat_map(|(_, a)| a).cloned().collect();
        let mut mapping = HashMap::<Ingredient, HashSet<Allergen>>::new();
        for (ings, algs) in foods {
            let ings: HashSet<Ingredient> = ings.iter().cloned().collect();
            for alg in algs {
                let ings_old = mapping
                    .entry(alg.to_string())
                    .or_insert_with(|| ings.clone());
                *ings_old = ings_old.intersection(&ings).cloned().collect();
            }
        }

        let mut all_singles = HashSet::<Ingredient>::new();
        loop {
            let singles = mapping
                .values()
                .filter_map(extract_singleton)
                .collect::<HashSet<_>>()
                .difference(&all_singles)
                .cloned()
                .collect::<HashSet<_>>();
            for (_, algs) in mapping.iter_mut() {
                if 1 < algs.len() {
                    *algs = algs.difference(&singles).cloned().collect();
                }
            }
            if singles.is_empty() {
                break;
            }
            all_singles = all_singles.union(&singles).cloned().collect();
        }

        Foods {
            ingredients,
            allergens,
            mapping,
        }
    }
}

fn solve(foods: &Foods) -> usize {
    foods
        .ingredients
        .iter()
        .filter_map(|(ing, cnt)| {
            if foods.mapping.values().all(|ings| !ings.contains(ing)) {
                Some(cnt)
            } else {
                None
            }
        })
        .sum()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p21.txt");
    let foods = input.parse::<Foods>()?;
    let out1 = solve(&foods);
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let foods = Foods::new(&[
            (
                vec![
                    "mxmxvkd".into(),
                    "kfcds".into(),
                    "sqjhc".into(),
                    "nhms".into(),
                ],
                vec!["dairy".into(), "fish".into()],
            ),
            (
                vec![
                    "trh".into(),
                    "fvjkl".into(),
                    "sbzzf".into(),
                    "mxmxvkd".into(),
                ],
                vec!["dairy".into()],
            ),
            (vec!["sqjhc".into(), "fvjkl".into()], vec!["soy".into()]),
            (
                vec!["sqjhc".into(), "mxmxvkd".into(), "sbzzf".into()],
                vec!["fish".into()],
            ),
        ]);
        assert_eq!(solve(&foods), 5);
    }
}
