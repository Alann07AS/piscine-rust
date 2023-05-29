pub fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }
    let mut result: u64 = 1;
    for i in 1..=num {
        result *= i
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let result = factorial(0);
        assert_eq!(result, 1 );
    }
    #[test]
    fn it_works1() {
        let result = factorial(1);
        assert_eq!(result, 1 );
    }
    #[test]
    fn it_works120() {
        let result = factorial(5);
        assert_eq!(result, 120 );
    }
    #[test]
    fn it_works3628800() {
        let result = factorial(10);
        assert_eq!(result, 3628800 );
    }
    #[test]
    fn it_works121645100408832000() {
        let result = factorial(19);
        assert_eq!(result, 121645100408832000 );
    }
}

