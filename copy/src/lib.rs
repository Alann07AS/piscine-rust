pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c = c as f64;
    let exp = c.exp();
    (c as i32, exp, c.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    (a.clone(), a.split_whitespace()
    .map(|x| x.parse().unwrap())
    .map(|x: i32| (x as f64).exp())
    .map(|x| x.to_string())
    .collect::<Vec<String>>().join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (b.clone(), b.clone().iter().map(|x| (*x as f64).ln()).collect::<Vec<f64>>())
}
