pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    // let v_len = v.chars().count();
    let mut result: Vec<String> = vec![];
    for i in 1..=i {
        result.push(
            format!("{}{}", String::from(" ").repeat(i as usize), v.repeat(i as usize))
        );
    }
    let mut rev_vec = result.clone();
    rev_vec.pop();
    rev_vec.reverse();
    result.append(&mut rev_vec);
    result
}