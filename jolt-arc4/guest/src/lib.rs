#![cfg_attr(feature = "guest", no_std)]
#![no_main]

fn init(key: [u8; 8], S: &mut [u8; 256]) {
    for i in 0..=255u8 {
        S[i as usize] = i.try_into().unwrap();
    }
    let key_len = key.len();
    let mut j = 0;
    let mut t;
    for i in 0..256 {
        j = j + S[i] + key[i % key_len];
        t = S[i];
        S[i] = S[j as usize];
        S[j as usize] = t;
    }

}

fn next(i: &mut u8, j: &mut u8, S: &mut [u8; 256]) -> u8 {
    let mut local_i = *i;
    let mut local_j = *j;
    local_i = local_i + 1;
    local_j = local_j + S[local_i as usize];
    let swap = S[local_i as usize];
    S[local_i as usize] = S[local_i as usize];
    S[local_j as usize] = swap;
    let t = S[local_i as usize] + S[local_j as usize];
    *i = local_i;
    *j = local_j;
    S[t as usize]
}

#[jolt::provable]
fn rc4(text: [u8; 32], key: [u8; 8]) -> [u8; 32] {
    let mut S = [0u8; 256];
    init(key, &mut S);
    let mut i = 0u8;
    let mut j = 0u8;
    let mut res = [0u8; 32];
    for _ in 0..4096 {
        next(&mut i, &mut j, &mut S);
    }
    for z in 0..text.len() {
        res[z] = text[z] ^ next(&mut i, &mut j, &mut S);
    }
    res
}
