pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut arr = Vec::<u64>::from(arr);
    let mut result = Vec::<u64>::new();
    while arr.len() != 0 {
        result.push(arr.iter().sum::<u64>());
        arr.pop();
    }
    result.push(0);
    result
}