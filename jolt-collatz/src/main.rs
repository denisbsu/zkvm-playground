use jolt::Proof;

pub fn main() {
    let (prove_collatz, verify_collatz) = guest::build_collatz_bench();
    use std::time::Instant;
    let iterations = 10;
    let now = Instant::now();

    let mut proofs = Vec::<Proof>::new();
    for _ in 0..iterations {
        let (_, proof) = prove_collatz(931_386_509_544_713_451, 10);
        proofs.push(proof);
    }
    let elapsed_prove = now.elapsed();

    let (output, proof) = prove_collatz(931_386_509_544_713_451, 10);
    proof.save_to_file("proof.bin").unwrap();

    let now = Instant::now();
    for _ in 0..iterations {
        _ = verify_collatz(proofs.pop().unwrap());
    }
    let elapsed_verify = now.elapsed();


    println!("output: {}", output);
    println!("Elapsed prove: {:.2?}", elapsed_prove / iterations);
    println!("Elapsed verify: {:.2?}", elapsed_verify / iterations);

    
}
