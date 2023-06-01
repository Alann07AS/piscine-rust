use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut h_map: HashMap<&str, usize> = HashMap::new();
    words.iter().for_each(|v| {
        *h_map.entry(v).or_insert(0)+=1;
    });
    h_map
    // let sum = arr.into_iter().reduce(|a, b| {
    //     a + b
    // } ).unwrap();
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
    // if frequency_count.is_empty() {return 0;}
    // frequency_count.iter().fold(0, |acc, (_, value)| {
    //     if *value == 1usize {
    //         acc + 1
    //     } else {
    //         acc
    //     }
    // })
    
    // .reduce(|(_, value_acc), (key, value)| {
    //     if value == 1 {
    //         value_acc + 1
    //     } else {
    //         value_acc
    //     }
    // } ).unwrap()
}