pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    Box::new(
        s.split_whitespace().into_iter().map(|value| {
            let is_k = value.chars().last().unwrap() == 'k';
            (&value[..value.len()-if is_k {1} else {0}].parse::<f64>().unwrap() * if is_k {1000.} else {1.}) as u32
        }).collect()
    )
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}