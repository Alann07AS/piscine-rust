pub use edit_distance::edit_distance;

pub fn expected_variable (to_compare: &str, expected: &str) -> Option<String> {
    if  !to_compare.chars().any(|ch| ch.is_ascii_uppercase()) && 
        !to_compare.contains("_") || 
        to_compare.contains(" ") {
            return None;
        }


    let to_compare = &to_compare.to_lowercase();
    let expected = &expected.to_lowercase();

    let len_expect = expected.chars().count().max(to_compare.chars().count()) as f64;
    let result_dif = 
    ((len_expect - edit_distance(to_compare, expected) as f64)/
    len_expect) * 100.;

    if result_dif < 50. {
        print!("UNDER 50");
        return None;
    }

    Some(format!(
        "{}%",
        result_dif.round()
    ))
        
}