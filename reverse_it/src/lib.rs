pub fn reverse_it(v: i32) -> String {
    let is_neg = v < 0;
    let mut st: String = v.to_string();
    if is_neg {st.remove(0);};
    let chars = st.chars().collect:: <Vec<char>>();
    let st_rev: String = chars.iter().rev().collect::<String>();
    (if is_neg {String::from("-")} else {String::from("")} + &st_rev + &st).to_string()
}