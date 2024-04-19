//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    use std::time::Instant;
    let now = Instant::now();
    let iterations = 1;
    // for _ in 0..iterations {
        let mut stdin = SP1Stdin::new();
        let n = 0u8;
        for _ in 0..32 {
            stdin.write(&n);
        }
        for i in 0..8u8 {
            stdin.write(&i)
        }
        let client = ProverClient::new();
        let mut proof = client.prove(ELF, stdin).expect("proving failed");
    // }
    let elapsed_prove = now.elapsed();

    // Read output.
    // let res = proof.public_values.read::<u128>();
    // println!("res: {}", res);

    // // Verify proof.
    client.verify(ELF, &proof).expect("verification failed");
    let elapsed_verify = now.elapsed();
    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!");
    println!("Elapsed prove: {:.2?}", elapsed_prove / iterations);
    println!("Elapsed verify: {:.2?}", elapsed_verify - elapsed_prove);
}
