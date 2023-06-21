pub fn num_to_ordinal(x: u32) -> String {
    x.to_string() +
    match x {
        11 | 12 | 13 => "th",
        n => match n%10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
    }
}