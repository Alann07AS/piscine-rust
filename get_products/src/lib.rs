pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return vec![];
    }
    arr.iter().enumerate().map(|(ignored_index , _)| {
        arr.iter().enumerate().fold(1, |acc: usize, (index, value)| {
            if index == ignored_index {
                acc * 1
            } else {
                acc * *value
            }
        })
    }).collect()
}
