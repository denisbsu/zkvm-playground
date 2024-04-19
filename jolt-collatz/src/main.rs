use jolt::Proof;

pub fn main() {
    let (prove_collatz, verify_collatz) = guest::build_collatz_bench();
    use std::time::Instant;
    let now = Instant::now();
    // let mut proof = Proof{proof};
    let mut output: u128 = 0;
    for _ in 0..10 {
        let (output, proof) = prove_collatz(931_386_509_544_713_451, 10);
    }
    let elapsed_prove = now.elapsed();
    // proof.save_to_file("proof.bin").unwrap();
    // let is_valid = verify_collatz(proof);
    let elapsed_verify = now.elapsed();


    // println!("output: {}", output);
    // println!("valid: {}", is_valid);
    println!("Elapsed prove: {:.2?}", elapsed_prove / 10);
    println!("Elapsed verify: {:.2?}", elapsed_verify - elapsed_prove);

    
}
