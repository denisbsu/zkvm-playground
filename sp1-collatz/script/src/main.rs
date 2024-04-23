//! A simple script to generate and verify the proof of a given program.

use std::iter;

use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");




fn main() {
    // Generate proof.
    use std::time::Instant;
    let now = Instant::now();
    let iterations = 10;
    let client = ProverClient::new();
    for _ in 0..iterations {
        let mut stdin = SP1Stdin::new();
        let n: u128 = 931_386_509_544_713_451;
        let iters: usize = 10;
        stdin.write(&n);
        stdin.write(&iters);
        _ = client.prove(ELF, stdin).expect("proving failed");
    }
    let elapsed_prove = now.elapsed();
    
    let mut stdin = SP1Stdin::new();
    let n: u128 = 931_386_509_544_713_451;
    let iters: usize = 10;
    stdin.write(&n);
    stdin.write(&iters);
    let mut proof = client.prove(ELF, stdin).expect("proving failed");

    // Read output.
    let res = proof.public_values.read::<u128>();
    println!("res: {}", res);

    // // Verify proof.
    let now = Instant::now();
    for _ in 0..iterations {
        client.verify(ELF, &proof).expect("verification failed");
    }
    let elapsed_verify = now.elapsed();
    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!");
    println!("Elapsed prove: {:.2?}", elapsed_prove / iterations);
    println!("Elapsed verify: {:.2?}", elapsed_verify / iterations);
}
