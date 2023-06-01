pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let source_len = source_chars.len();
    let target_len = target_chars.len();

    // Create a 2D matrix to store the distances
    let mut distances = vec![vec![0; target_len + 1]; source_len + 1];

    // Initialize the first row and column
    for i in 0..=source_len {
        distances[i][0] = i; // Distance entre la sous-chaîne vide de source et les préfixes de target
    }
    for j in 0..=target_len {
        distances[0][j] = j; // Distance entre la sous-chaîne vide de target et les préfixes de source
    }

    // Calculate the distances
    for i in 1..=source_len {
        for j in 1..=target_len {
            let substitution_cost = if source_chars[i - 1] == target_chars[j - 1] {
                0 // Pas de coût de substitution si les caractères sont identiques
            } else {
                1 // Coût de substitution de 1 si les caractères sont différents
            };

            // Mise à jour de la distance en choisissant le coût minimal parmi les trois opérations possibles
            distances[i][j] = (distances[i - 1][j] + 1) // Coût d'insertion
                .min(distances[i][j - 1] + 1) // Coût de suppression
                .min(distances[i - 1][j - 1] + substitution_cost); // Coût de substitution
        }
    }

    // Return the Levenshtein distance
    distances[source_len][target_len] // Distance entre les sous-chaînes complètes de source et target
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