use std::collections::HashMap;

pub fn slices_to_map<'a, U, T: Eq + std::hash::Hash>(t: &'a [T],u: &'a [U]) -> HashMap<&'a T, &'a  U> {
    let max = t.len().max(u.len());
    let keys = &t[..max];
    let values = &u[..max];
    keys.iter().zip(values.iter()).collect()
}