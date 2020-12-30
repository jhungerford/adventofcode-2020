use day21::{load_food, count_non_allergens, dangerous_ingredients};

fn main() {
    let food = load_food("input.txt");

    println!("Part 1: {}", count_non_allergens(&food));
    println!("Part 2: {}", dangerous_ingredients(&food));
}
