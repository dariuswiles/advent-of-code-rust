//! Advent of Code 2020 Day 21
//! https://adventofcode.com/2020/day/21
//!
//! Challenge part 1
//!
//! Given an input file listing foods with ingredient and incomplete allergen information,
//! determine the number of times ingredients which are free from all allergens appear in the list
//! of foods.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

const INPUT_FILENAME: &str = "2020_day21_input.txt";
const INPUT_DELIMITER: &str = " (contains ";

type Ingredient<'a> = &'a str;
type Allergen<'a> = &'a str;

#[derive(Clone, Debug, PartialEq)]
struct TokenizedInput<'a> {
    foods: Vec<(HashSet<Ingredient<'a>>, HashSet<Allergen<'a>>)>
}

impl<'a> TokenizedInput<'a> {
    /// Parses the challenge `input` into a `Vec` containing an entry for each line in the input
    /// file. Each line describes one food. For each food the `Vec` is a tuple of the `HashSet` of
    /// all ingredients in contains and a `HashSet` of its associated allergens.
    fn parse_input(input: &'a str) -> Self {
        let mut foods = vec![];

        for row in input.lines() {
            if row == "" {
                continue;
            }

            let ingredients_allergens: Vec<&str> = row.split(INPUT_DELIMITER).collect();

            if ingredients_allergens.len() != 2 {
                panic!("Row lacks expected delimiter between ingredients and allergens: {}", &row);
            }

            let ingredients: HashSet<&str> = ingredients_allergens[0].split(' ').collect();
            let allergens: HashSet<&str> = ingredients_allergens[1].strip_suffix(')').unwrap()
                .split(", ").collect();

            foods.push((ingredients, allergens));
        }

        Self { foods }
    }

    /// Returns a `Vec` containing every ingredient listed in the input file. Ingredients that
    /// occur multiple times in the input file appear the same number of times in this function's
    /// output. The output is sorted alphabetically.
    fn all_ingredients(&self) -> Vec<&Ingredient> {
        let mut result: Vec<&Ingredient> = self.foods.iter().map(|(i, _)| i).flatten().collect();
        result.sort_unstable();
        result
    }

    /// Returns a `Vec` containing all ingredients listed in the input file minus the foods
    /// passed in `allergic_ingredients`. Ingredients that occur multiple times in the input file
    /// appear the same number of times in this function's output. The output is sorted
    /// alphabetically.
    fn safe_ingredients(&self, allergic_ingredients: &Vec<&Ingredient>) -> Vec<&Ingredient> {
        let mut safe_ingredients = self.all_ingredients().clone();
        safe_ingredients.retain(|i| !allergic_ingredients.contains(&i));
        safe_ingredients
    }
}


/// An `IngredientSets` object represents all the data in a challenge input file, but organized
/// so that allergens are the primary key. This allows is to simplify subsequent processing.
#[derive(Clone, Debug, PartialEq)]
struct IngredientSets<'a> {
    sets: HashMap<&'a str, Vec<HashSet<&'a str>>>
}

impl<'a> IngredientSets<'a> {
    /// Parses the tokenized `input` into a `HashMap` containing an entry for each allergen. The
    /// value of each entry is a `Vec` of sets of ingredients (stored as a `HashSet`). For example,
    /// 'soy' may map to two foods, one containing 'abc' and 'def', and the other containing 'mno',
    /// 'pqr' and 'stu'.
    fn map_allergens(input: &'a TokenizedInput) -> Self {
        let mut allergens_to_ingredients: HashMap<&str, Vec<HashSet<&str>>> = HashMap::new();

        for (ingredients, allergens) in &input.foods {
            for allergen in allergens {
                if let Some(a2i) = allergens_to_ingredients.get_mut(allergen) {
                    a2i.push(ingredients.clone());
                } else {
                    allergens_to_ingredients.insert(&allergen, vec![ingredients.clone()]);
                }
            }
        }

        Self { sets: allergens_to_ingredients }
    }
}


/// An `AllergenMapTransition` is a transition object used to determine the unique mapping between
/// each allergen and the one ingredient that contains it. The object maps each allergen to the set
/// of ingredients that it could be in. A method is provided to iteratively narrow this down until
/// each allergen maps to exactly one ingredient.
#[derive(Clone, Debug, PartialEq)]
struct AllergenMapTransition<'a> {
    map: HashMap<&'a str, HashSet<&'a str>>
}

impl<'a> AllergenMapTransition<'a> {
    /// Reduce the `IngredientSets` input so that each allergen maps to the set of ingredients it
    /// could be in.
    fn new(ingredient_sets: &'a IngredientSets) -> Self {
        let mut map = HashMap::new();
            for (allergen, ingredients) in &ingredient_sets.sets {
                map.insert(*allergen, ingredients.clone().iter().fold(ingredients[0].clone(),
                    |acc, hs| acc.intersection(hs).cloned().collect::<HashSet<&str>>()
                ));
            }

        Self { map }
    }

    /// Repeatedly iterates over the map of allergens to ingredients until each allergen has
    /// exactly one ingredient. Returns a `HashMap` of this allergen to ingredient mapping.
    /// Consumes this object as all useful data is moved into the result returned.
    ///
    /// # Panics
    ///
    /// The challenge states that each allergen maps to exactly one ingredient, but if such a
    /// mapping cannot be found, the function panics.
    fn solve(mut self) -> HashMap<&'a str, &'a str> {
        let mut solved_allergens: HashMap<&str, &str> = HashMap::new();

        while solved_allergens.len() < self.map.len() {
            let mut solved_this_turn: HashSet<&str> = HashSet::new();

            for (allergen, ingredients) in &self.map {
                if solved_allergens.get(allergen) != None {
                    continue;
                }

                if ingredients.len() == 1 {
                    solved_this_turn.insert(*ingredients.iter().nth(0).unwrap());
                    solved_allergens.insert(allergen, *ingredients.iter().nth(0).unwrap());
                }
            }

            assert!(solved_this_turn.len() != 0, "Could not uniquely map allergens to ingredients");

            for (allergen, ingredients) in self.map.clone() {

                let new_ingredients: HashSet<&str> = HashSet::from_iter(ingredients.difference(&solved_this_turn).cloned().collect::<Vec<&str>>());

//                 println!("new_ingredients for allergen {} are\n{:#?}", &allergen, &new_ingredients);

                let tmp = self.map.get_mut(allergen).unwrap();
                *tmp = new_ingredients;
            }
        }
        solved_allergens
    }
}


fn do_challenge(input: &str) -> usize {
    let foods = TokenizedInput::parse_input(input);
    let ing_sets = IngredientSets::map_allergens(&foods);
    let initial_mapping = AllergenMapTransition::new(&ing_sets);
    let mapping = initial_mapping.solve();
    let unsafe_ingredients = mapping.values().collect();
    let safe_ingredients = foods.safe_ingredients(&unsafe_ingredients);

    safe_ingredients.len()
}


fn main() {
    let input_file =
        fs::read_to_string(INPUT_FILENAME)
            .expect("Error reading input file");

    let answer = do_challenge(&input_file);

    println!("Allergen-free ingredients appear in the list of foods {} times", answer);
}


// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_parse_input() {
        let foods = TokenizedInput::parse_input(&TEST_INPUT);
        let ing_sets = IngredientSets::map_allergens(&foods);

        assert_eq!(ing_sets.sets["dairy"][0], ["mxmxvkd", "kfcds", "sqjhc", "nhms"].iter().cloned()
            .collect::<HashSet<_>>()
        );
        assert_eq!(ing_sets.sets["dairy"][1], ["trh", "fvjkl", "sbzzf", "mxmxvkd"].iter().cloned()
            .collect::<HashSet<_>>()
        );
        assert_eq!(ing_sets.sets["fish"][0], ["mxmxvkd", "kfcds", "sqjhc", "nhms"].iter().cloned()
            .collect::<HashSet<_>>()
        );
        assert_eq!(ing_sets.sets["fish"][1], ["sqjhc", "mxmxvkd", "sbzzf"].iter().cloned()
            .collect::<HashSet<_>>()
        );
        assert_eq!(ing_sets.sets["soy"][0], ["sqjhc", "fvjkl"].iter().cloned()
            .collect::<HashSet<_>>());
    }

    #[test]
    fn initial_mapping() {
        let foods = TokenizedInput::parse_input(&TEST_INPUT);
        let ing_sets = IngredientSets::map_allergens(&foods);
        let initial_mapping = AllergenMapTransition::new(&ing_sets);

        println!("{:#?}", initial_mapping);

        assert_eq!(initial_mapping.map["soy"], ["fvjkl", "sqjhc"].iter().cloned()
            .collect::<HashSet<_>>());
        assert_eq!(initial_mapping.map["dairy"], ["mxmxvkd"].iter().cloned()
            .collect::<HashSet<_>>());
        assert_eq!(initial_mapping.map["fish"], ["sqjhc", "mxmxvkd"].iter().cloned()
            .collect::<HashSet<_>>());
    }

    #[test]
    fn determine_allergen_to_ingredient_map() {
        let foods = TokenizedInput::parse_input(&TEST_INPUT);
        let ing_sets = IngredientSets::map_allergens(&foods);
        let initial_mapping = AllergenMapTransition::new(&ing_sets);
        let mapping = initial_mapping.solve();

        assert_eq!(mapping["dairy"], "mxmxvkd");
        assert_eq!(mapping["fish"], "sqjhc");
        assert_eq!(mapping["soy"], "fvjkl");
    }

    #[test]
    fn test_all_ingredients() {
        let foods = TokenizedInput::parse_input(&TEST_INPUT);
        let ing_sets = IngredientSets::map_allergens(&foods);
        let initial_mapping = AllergenMapTransition::new(&ing_sets);

        let mut all_ingredients = foods.all_ingredients();

        let expected = vec![
            &"fvjkl",
            &"fvjkl",
            &"kfcds",
            &"mxmxvkd",
            &"mxmxvkd",
            &"mxmxvkd",
            &"nhms",
            &"sbzzf",
            &"sbzzf",
            &"sqjhc",
            &"sqjhc",
            &"sqjhc",
            &"trh",
        ];

        assert_eq!(expected, all_ingredients);
    }

    #[test]
    fn safe_ingredients() {
        let foods = TokenizedInput::parse_input(&TEST_INPUT);
        let ing_sets = IngredientSets::map_allergens(&foods);
        let initial_mapping = AllergenMapTransition::new(&ing_sets);
        let mapping = initial_mapping.solve();
        let unsafe_ingredients = mapping.values().collect();
        println!("unsafe_ingredients = {:#?}", unsafe_ingredients);

        let safe_ingredients = foods.safe_ingredients(&unsafe_ingredients);
        println!("safe_ingredients = {:#?}", safe_ingredients);

        let expected = vec![
            &"kfcds",
            &"nhms",
            &"sbzzf",
            &"sbzzf",
            &"trh",
        ];

        assert_eq!(expected, safe_ingredients);
    }

}
