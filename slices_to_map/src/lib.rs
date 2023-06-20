use std::collections::HashMap;

pub fn slices_to_map<'a, U, T: Eq + std::hash::Hash>(t: &'a [T],u: &'a [U]) -> HashMap<&'a T, &'a  U> {
    let min = t.len().min(u.len());
    let keys = &t[..min];
    let values = &u[..min];
    keys.iter().zip(values.iter()).collect()
}