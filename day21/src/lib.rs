use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseErr {
    Invalid
}

#[derive(Debug, Eq, PartialEq)]
pub struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // s is a set of ingredients separated by spaces, followed by a list of comma space
        // separated allergens like '(contains dairy, fish)'
        let contains_index = s.find(" (contains ").unwrap();

        let ingredients: Vec<String> = s[0..contains_index].split(" ")
            .map(str::to_owned)
            .collect();

        let allergen_start = contains_index + 11;
        let allergens: Vec<String> = s[allergen_start .. s.len() - 1].split(", ")
            .map(str::to_owned)
            .collect();

        Ok(Food { ingredients, allergens })
    }
}

/// Loads a list of food from the given file.
pub fn load_food(filename: &str) -> Vec<Food> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

/// Counts the number of allergens that can't contain any allergens in the list of food.
pub fn count_non_allergens(foods: &Vec<Food>) -> usize {
    // Allergen ingredient must show up in all of the foods.
    // Ingredient can't belong to multiple allergens.
    // Foods may have allergens that aren't labeled.

    // Start with a map of allergen -> ingredients, where ingredients are the intersection
    // of all foods that contain the allergen.
    // Allergens that have one ingredient are identified - remove them from the possibilities
    // of other allergens
    // Iterate until no more allergens have been identified.
    // Number of known non-allergens is # unique ingredients - # sus ingredients

    let mut allergen_foods = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            allergen_foods.entry(allergen).or_insert(Vec::new()).push(food);
        }
    }

    let mut potential_allergens: HashSet<&String> = HashSet::new();
    for &allergen in allergen_foods.keys() {
        // Intersection of ingredients for all foods with an allergen labeled are
        // the possibilities for that allergen.
        let mut food_ingredients = allergen_foods.get(allergen).unwrap().iter()
            .map(|&food| HashSet::from_iter(food.ingredients.iter()));

        let first_ingredient: HashSet<&String> = food_ingredients.next().unwrap();

        let sus_ingredients = food_ingredients.fold(first_ingredient, |sus, ingredient| {
            sus.intersection(&ingredient).cloned().collect()
        });

        for ingredient in sus_ingredients {
            potential_allergens.insert(ingredient);
        }
    }

    foods.iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|&ingredient| !potential_allergens.contains(ingredient))
        .count()
}

/// Determines which ingredients are allergens in the foods, sorts them alphabetically by allergen,
/// and returns a comma-separated list of the allergen ingredients.
pub fn dangerous_ingredients(foods: &Vec<Food>) -> String {
    // Allergen ingredient must show up in all of the foods.
    // Ingredient can't belong to multiple allergens.
    // Foods may have allergens that aren't labeled.

    // Start with a map of allergen -> ingredients, where ingredients are the intersection
    // of all foods that contain the allergen.
    // Allergens that have one ingredient are identified - remove them from the possibilities
    // of other allergens
    // Iterate until no more allergens have been identified.
    // Number of known non-allergens is # unique ingredients - # sus ingredients

    let mut allergen_foods = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            allergen_foods.entry(allergen).or_insert(Vec::new()).push(food);
        }
    }

    let mut allergen_ingredients = HashMap::new();
    for &allergen in allergen_foods.keys() {
        // Intersection of ingredients for all foods with an allergen labeled are
        // the possibilities for that allergen.
        let mut food_ingredients = allergen_foods.get(allergen).unwrap().iter()
            .map(|&food| HashSet::from_iter(food.ingredients.iter()));

        let first_ingredient: HashSet<&String> = food_ingredients.next().unwrap();

        let sus = food_ingredients
            .fold(first_ingredient, |sus, ingredient| {
                sus.intersection(&ingredient).cloned().collect()
            });

        allergen_ingredients.insert(allergen, sus);
    }

    // Map of allergen -> ingredient.
    let mut identified_allergens: HashMap<&String, &String> = HashMap::new();
    loop {
        let mut new_allergens = Vec::new();
        let mut new_ingredients = Vec::new();
        for (&allergen, ingredients) in allergen_ingredients.iter() {
            if !identified_allergens.contains_key(allergen) && ingredients.len() == 1 {
                let ingredient = *ingredients.iter().next().unwrap();

                new_allergens.push(allergen);
                new_ingredients.push(ingredient);

                identified_allergens.insert(allergen, ingredient);
            }
        }

        for ingredients in allergen_ingredients.values_mut() {
            for &new_ingredient in &new_ingredients {
                ingredients.remove(new_ingredient);
            }
        }

        if new_allergens.is_empty() {
            break;
        }
    }

    let mut allergens: Vec<&String> = identified_allergens.keys().cloned().collect();
    allergens.sort();

    let mut ingredients_iter = allergens.iter().map(|allergen| identified_allergens.get(allergen).unwrap());

    let first_ingredient = ingredients_iter.next().unwrap().clone().clone();
    ingredients_iter.fold(first_ingredient, |str, ingredient| format!("{},{}", str, ingredient))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_food() {
        let expected = Food {
            ingredients: vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"].iter().map(|&s| s.to_owned()).collect(),
            allergens: vec!["dairy", "fish"].iter().map(|&s| s.to_owned()).collect(),
        };

        assert_eq!(Ok(expected), "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".parse())
    }

    #[test]
    fn load_sample() {
        let sample = load_food("sample.txt");

        assert_eq!(4, sample.len());

        assert_eq!(vec!["sqjhc", "mxmxvkd", "sbzzf"], sample[3].ingredients);
        assert_eq!(vec!["fish"], sample[3].allergens);
    }

    #[test]
    fn count_sample() {
        let sample = load_food("sample.txt");

        assert_eq!(5, count_non_allergens(&sample));
    }

    #[test]
    fn dangerous_sample() {
        let sample = load_food("sample.txt");

        assert_eq!("mxmxvkd,sqjhc,fvjkl", dangerous_ingredients(&sample));
    }
}