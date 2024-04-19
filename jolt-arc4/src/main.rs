use jolt::Proof;

pub fn main() {
    let (prove_rc4, verify_rc4) = guest::build_rc4();
    use std::time::Instant;
    let now = Instant::now();
    let iterations = 1;
    // let mut proof = Proof{proof};
    // let mut output: u128 = 0;
    // for _ in 0..iterations {
        let mut text = [0u8; 32];
        let key = [0u8, 1, 2, 3, 4, 5, 6, 7];
        let (output, proof) = prove_rc4(text, key);
    // }
    let elapsed_prove = now.elapsed();
    // proof.save_to_file("proof.bin").unwrap();
    let is_valid = verify_rc4(proof);
    let elapsed_verify = now.elapsed();


    // println!("output: {:?}", output);
    println!("valid: {}", is_valid);
    println!("Elapsed prove: {:.2?}", elapsed_prove / iterations);
    println!("Elapsed verify: {:.2?}", elapsed_verify - elapsed_prove);

    
}
