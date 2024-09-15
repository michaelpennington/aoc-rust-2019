use std::collections::HashMap;

use anyhow::anyhow;

advent_of_code::solution!(14);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Recipes<'a> {
    ingredients: HashMap<&'a str, Recipe<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Recipe<'a> {
    makes: u64,
    requires: Vec<(&'a str, u64)>,
}

impl<'a> Recipes<'a> {
    fn from_str(input: &'a str) -> anyhow::Result<Self> {
        let mut ingredients = HashMap::with_capacity(input.lines().count());
        let err = || anyhow!("'{input}' is not a valid recipe");
        for line in input.lines() {
            let (need, target) = line.split_once(" => ").ok_or_else(err)?;
            let mut requires = Vec::new();
            for item in need.split(", ") {
                let (num, ing) = item.split_once(' ').ok_or_else(err)?;
                let num = num.parse()?;
                requires.push((ing, num));
            }
            let (num, name) = target.split_once(' ').ok_or_else(err)?;
            let makes = num.parse()?;
            let recipe = Recipe { makes, requires };
            ingredients.insert(name, recipe);
        }
        Ok(Self { ingredients })
    }

    fn calc_ore(&self, target: &str, count: u64) -> u64 {
        let mut requirements = HashMap::new();
        requirements.insert(target, count);
        let mut extra: HashMap<&str, u64> = HashMap::new();
        loop {
            let mut new_requirements = HashMap::new();
            // Need `num` of ingredient `name`
            for (name, num) in requirements {
                // This is the recipe to make `name`.
                // it makes `recipe.count` copies
                // Amount of *recipes* we have to invoke is
                // num_recipe = ⌈(num - extra(name)) ÷ recipe.makes⌉
                // This will make `num_recipe * recipe.makes` copies of `name`
                // Extra left over is num_recipe * recipe.makes + extra(name) - num
                if name == "ORE" {
                    *new_requirements.entry("ORE").or_default() += num;
                } else {
                    let recipe = &self.ingredients[name];
                    let num_recipe = (num.saturating_sub(*extra.entry(name).or_default()))
                        .div_ceil(recipe.makes);
                    let ex = extra.get_mut(name).unwrap();
                    *ex += num_recipe * recipe.makes;
                    *ex -= num;
                    for (ingredient, amount) in &recipe.requires {
                        let amount_needed = amount * num_recipe;
                        *new_requirements.entry(*ingredient).or_default() += amount_needed;
                    }
                }
            }
            if new_requirements.len() == 1
                && new_requirements.keys().next().is_some_and(|k| *k == "ORE")
            {
                return new_requirements["ORE"];
            }
            requirements = new_requirements;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let recipes = Recipes::from_str(input).unwrap();
    Some(recipes.calc_ore("FUEL", 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let recipes = Recipes::from_str(input).unwrap();
    let mut max = 1_000_000_000;
    let mut min = 0;
    loop {
        let ore_per_fuel = recipes.calc_ore("FUEL", (max + min) / 2);
        if ore_per_fuel > 1_000_000_000_000 {
            max = (max + min) / 2;
        } else {
            min = (max + min) / 2;
        }
        if max == min || max == min + 1 {
            return Some(min);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(13312));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(180697));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(2210736));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(82892753));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(5586022));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(460664));
    }
}
