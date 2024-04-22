use jolt::Proof;

pub fn main() {
    let (prove_rc4, verify_rc4) = guest::build_rc4();
    use std::time::Instant;
    let iterations = 10;
    let mut proofs = Vec::<Proof>::new();
    let now = Instant::now();
    for _ in 0..iterations {
        let text = [0u8; 32];
        let key = [0u8, 1, 2, 3, 4, 5, 6, 7];
        let (_, proof) = prove_rc4(text, key);
        proofs.push(proof);
    }
    let elapsed_prove = now.elapsed();

    let text = [0u8; 32];
    let key = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let (output, proof) = prove_rc4(text, key);
    proof.save_to_file("proof.bin").unwrap();
    let now = Instant::now();
    for _ in 0..iterations {
        _ = verify_rc4(proofs.pop().unwrap());
    }
    let elapsed_verify = now.elapsed();


    println!("output: {:?}", output);
    println!("Elapsed prove: {:.2?}", elapsed_prove / iterations);
    println!("Elapsed verify: {:.2?}", elapsed_verify / iterations);

    
}
