use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    (list.iter().sum::<i32>() as f64) / (list.len() as f64)
}


pub fn median(list: &Vec<i32>) -> i32 {
    let l = list.len();
    if l%2 == 0 {
        (list[l/2]+list[(l/2)+1])/2
    } else {
        list[l/2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map_count = HashMap::new();
    list.iter().for_each(|nb| *map_count.entry(nb).or_insert(0)+=1);
    match map_count.iter().max_by_key(|(_, &value)| value) {
        None => 0,
        Some((key, _)) => **key,
    }
}