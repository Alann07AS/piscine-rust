pub fn first_fifty_even_square() -> Vec<i32> {
    (2..=100).step_by(2)
    .map(|valeur| valeur * valeur)
    .collect::<Vec<i32>>()
}