pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = vec![];
    let c_nb = c as u8 - 'A' as u8 + 1;
    let larg = vec![' '; (c_nb * 2) as usize];
    let curent_char = 'A' as u8;
    for i in 0..c_nb {
        let mut line = larg.clone();
        line[(c_nb-i) as usize] = (curent_char + i) as char;
        line[(c_nb+i) as usize] = (curent_char + i) as char;
        result.push(
            line[1..].into_iter().collect()
        );
    }
    let mut rev_result = result[..result.len()-1].to_owned();
    rev_result.reverse();
    result.extend(rev_result);
    result
}