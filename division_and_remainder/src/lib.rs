pub fn divide(x: i32, y: i32) -> (i32, i32) {
    return (x/y, x%y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(9, 4);
        assert_eq!(result, (2, 1));
    }
}
