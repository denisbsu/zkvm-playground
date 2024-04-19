#![no_main]
sp1_zkvm::entrypoint!(main);

fn collatz(n: u128) -> u128 {
    let mut seq_len = 0;
    let mut number = n;
    while number != 1 {
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = 3 * number + 1;
        }
        seq_len += 1;
    }
    seq_len
}

pub fn main() {
    let n = sp1_zkvm::io::read::<u128>();
    let iters = sp1_zkvm::io::read::<usize>();
    let mut res = 0;
    for _ in 0..iters {
        res = collatz(n);
    }
    sp1_zkvm::io::commit(&res);
}
