fn main() {
    println!("c'est premier: {}",is_prime(4));
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 || n == 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let mut w = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += w;
        w = 6 - w;
    }

    true
}