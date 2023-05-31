use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    (list.iter().sum::<i32>() as f64) / (list.len() as f64)
}


pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let len = sorted_list.len();
    if len % 2 == 0 { // pair
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (sorted_list[mid_left] + sorted_list[mid_right]) / 2
    } else { // impair
        sorted_list[len / 2]
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