use std::io;

fn main() {
    let mut try_count: u64 = 0;
    loop {
        try_count+=1;
        println!("{}", 
        "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "The letter e" {
            break;
        }
    }
    println!{"Number of trials: {}", try_count}
}
