// use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let stats: (f64, f64, f64, f64) = (
        foods.iter().fold(0., |acc: f64, food: &Food| food.calories[1].trim_end_matches("kcal").parse().unwrap_or(0.) * food.nbr_of_portions + acc ),
        foods.iter().fold(0., |acc: f64, food: &Food| food.carbs * food.nbr_of_portions + acc ),
        foods.iter().fold(0., |acc: f64, food: &Food| food.proteins * food.nbr_of_portions + acc ),
        foods.iter().fold(0., |acc: f64, food: &Food| food.fats * food.nbr_of_portions + acc ),
    );

    json::object! {
        "cals" => round_two(stats.0),
        "carbs" => round_two(stats.1),
        "proteins" => round_two(stats.2),
        "fats" => round_two(stats.3),
    }
} 

fn round_two(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}
