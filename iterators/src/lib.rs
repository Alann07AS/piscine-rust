#[derive(Copy, Clone, Debug)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    fn next(&mut self) -> Option<Self::Item> {
            println!("Collatz value before next: {}", self.v);
            let t = if self.v < 2 {
                None
            } else {

                Some(Collatz::new(
                    if self.v%2 == 0 {
                        self.v / 2
                    } else {
                        self.v * 3 + 1
                    }
                ))
            };
            println!("Collatz value after next: {}", self.v);
            t
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        println!("New Collatz value: {}", n);
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut n = Collatz::new(n);

    let mut cp = 0;
    
    while n.next().is_some() {
        n = n.next().unwrap();
        cp += 1;
    }
    cp
}

// let log = []
// let temp = 1

// if (nb%2 == 0) {
// log.push(mul2); temp *=2
// for (;(nb - temp)%4 != 0;) {
//     log.push(mul2); temp *=2
// }
// for (let i = (nb-temp)/4; i > 0 ; i--) {
//     log.push(add4); temp += 4
// }
// return '1 ' + log.join(' ')
// }
// if ((nb-1)%4 == 0) {
// for (;temp != nb;) {
//     log.push(add4); temp += 4
// }
// return '1 ' + log.join(' ')
// }
// return undefined