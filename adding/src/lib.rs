pub fn add_curry(ref_val: i64) -> impl Fn(i64) -> i64 {
    move |val| val + ref_val
}