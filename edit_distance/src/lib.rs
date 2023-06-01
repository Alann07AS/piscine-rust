pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let source_len = source_chars.len();
    let target_len = target_chars.len();

    // Create a 2D matrix to store the distances
    let mut distances = vec![vec![0; target_len + 1]; source_len + 1];

    // Initialize the first row and column
    for i in 0..=source_len {
        distances[i][0] = i;
    }
    for j in 0..=target_len {
        distances[0][j] = j;
    }

    // Calculate the distances
    for i in 0..source_len {
        for j in 0..target_len {
            let substitution_cost = if source_chars[i] == target_chars[j] {
                0
            } else {
                1
            };

            distances[i][j] = (distances[i - 1][j] + 1)
                .min(distances[i][j - 1] + 1)
                .min(distances[i - 1][j - 1] + substitution_cost);
        }
    }

    // Return the Levenshtein distance
    distances[source_len][target_len]
}
// use std::collections::HashMap;

// pub fn edit_distance(source: &str, target: &str) -> usize {
//     let mut op_coumpt: usize = 0;
//     let mut letter_coupt_source: HashMap<char, usize> = HashMap::new(); 
//     source.chars().map(|ch| letter_coupt_source
//         .insert(ch, 
//             match letter_coupt_source.get(&ch) {
//                 Some(n) => *n,
//                 None => 0,
//             } + 1)); 
//     let mut letter_coupt_target: HashMap<char, usize> = HashMap::new(); 
//     target.chars().map(|ch| letter_coupt_target
//         .insert(ch, 
//             match letter_coupt_target.get(&ch) {
//                 Some(n) => *n,
//                 None => 0,
//             } + 1
//         ));
    
//     letter_coupt_target.iter().enumerate().map(
//         |(ch, c_target)| {
//             if letter_coupt_source.contains_key(ch) {
//                 let c_source = letter_coupt_source.get(ch).unwrap();
//                 let dif = c_target - c_source;

//             } else {
//                 add
//             }
//         }
//     );
//     // while source != target {
        
//     // }
//     op_coumpt
// }
// // insertions, deletions and/or substitutions

// // "alignment"
// // "assignment"