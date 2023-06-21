pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let sum = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits))
        .sum::<u32>();

    sum == num
}