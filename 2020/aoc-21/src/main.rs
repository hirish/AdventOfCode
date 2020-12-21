use aoc::read_stdin;
use std::collections::{HashSet, HashMap};


fn is_solved(potential_ingredients: &HashMap<String, HashSet<String>>) -> bool {
    for potentials in potential_ingredients.values() {
        if potentials.len() > 1 {
            return false
        }
    }
    true
}


fn part_1(input: &str) -> usize {
    let mut potential_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut recipes: Vec<Vec<&str>> = vec![];
    let input = input.replace(")", "");

    for line in input.lines() {
        let mut s = line.split(" (contains ");
        let ingredients: HashSet<&str> = s
            .next().unwrap()
            .split(" ")
            .collect();

        recipes.push(ingredients.iter().cloned().collect());

        let allergens: Vec<&str> = s
            .next().unwrap()
            .split(", ")
            .collect();

        for allergen in allergens {
            let mut x = ingredients.clone();
            if let Some(existing) = potential_ingredients.get(&allergen) {
                x = ingredients
                    .intersection(existing)
                    .map(|x| x.clone())
                    .collect();
            }
            potential_ingredients.insert(allergen, x);
        }
    }

    let mut solved_allergens: HashMap<&str, &str> = HashMap::new();

    while potential_ingredients.len() != solved_allergens.len() {
        for (allergen, ingredients) in potential_ingredients.iter() {
            if ingredients.len() == 1 {
                solved_allergens.insert(allergen, ingredients.iter().next().unwrap());
            };
        }

        for ingredient in solved_allergens.values() {
            for ingredients in potential_ingredients.values_mut() {
                ingredients.remove(ingredient);
            }
        }
    }

    let solved_ingredients: HashSet<&str> = solved_allergens.values().map(|&x| x).collect();
    let mut allergens: Vec<&str> = solved_allergens.keys().map(|x| *x).collect();
    allergens.sort();
    let ingredients: Vec<&str> = allergens.iter().map(|a| *solved_allergens.get(a).unwrap()).collect();

    // Part 2
    dbg!(&ingredients);

    recipes
        .iter()
        .map(|r| r
             .iter()
             .filter(|i| !solved_ingredients.contains(*i))
             .count()
        )
        .sum()
}


fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(&input));
}
