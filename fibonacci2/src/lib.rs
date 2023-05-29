pub fn fibonacci(n: u32) -> u32 {
    recu_fibo(n)
}

fn recu_fibo(index: u32) -> u32 {
    if index == 0 {
        return 0;
    } else if index == 1 {
        return 1;
    } else {
        return recu_fibo(index-1) + recu_fibo(index-2);
    }
    // Fn – 1 + Fn – 2
}