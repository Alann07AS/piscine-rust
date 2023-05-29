pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    let third: Option<&String> = vec.get(index);
    match third {
        Some(third) => third,
        None => "",
    }.to_string()
}
