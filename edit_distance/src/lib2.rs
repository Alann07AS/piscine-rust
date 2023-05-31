use std::collections::HashMap;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut op_coumpt: usize = 0;
    let mut letter_coupt_source: HashMap<char, usize> = HashMap::new(); 
    source.chars().map(|ch| letter_coupt_source
        .insert(ch, 
            match letter_coupt_source.get(&ch) {
                Some(n) => *n,
                None => 0,
            } + 1)); 
    let mut letter_coupt_target: HashMap<char, usize> = HashMap::new(); 
    target.chars().map(|ch| letter_coupt_target
        .insert(ch, 
            match letter_coupt_target.get(&ch) {
                Some(n) => *n,
                None => 0,
            } + 1
        ));
    
    target.iter().enumerate().map(|(ch, coupt)|)
    // while source != target {
        
    // }
    op_coumpt
}
// insertions, deletions and/or substitutions

// "alignment"
// "assignment"