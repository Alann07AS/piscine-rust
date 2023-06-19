pub use adding::add_curry;

pub fn twice<T: Fn(i64) -> i64>(f: T) -> impl Fn(i64) -> i64 {
    move |val| f(f(val))
}   