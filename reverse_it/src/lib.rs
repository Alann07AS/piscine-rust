pub fn reverse_it(v: i32) -> String {
    let mut st: String = v.to_string();
    let mut chars = st.chars().collect:: <Vec<char>>();
    let is_neg = chars[0] == '-';
    if is_neg {chars.remove(0); st.remove(0);};
    let st_rev: String = chars.iter().rev().collect::<String>();
    (if is_neg {String::from("-")} else {String::from("")} + &st_rev + &st).to_string()
}