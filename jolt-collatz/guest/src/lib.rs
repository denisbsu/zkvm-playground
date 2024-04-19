#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
fn collatz_bench(n: u128, iters: usize) -> u128 {
    let mut res = 0;
    for _ in 0..iters {
        res = collatz(n)
    }
    res
}

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
