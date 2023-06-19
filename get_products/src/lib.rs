pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return vec![];
    }
    arr.iter().enumerate().map(|(ignored_index , _)| {
        arr.iter().enumerate().fold(1, |acc: usize, (index, value)| {
            acc * if index == ignored_index {1} else {*value}
        })
    }).collect()
}
