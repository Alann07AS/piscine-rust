pub fn stars(n: u32) -> String {
    "*".to_string()
        .repeat(
            2_i32.pow(n) as usize
        )
}