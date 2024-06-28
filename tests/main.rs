use num_bigint::BigInt;
use rust_witness::witness;
use std::collections::HashMap;
use std::time::Instant;

fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::new();
    for &byte in bytes {
        for j in 0..8 {
            let bit = (byte >> j) & 1;
            bits.push(bit == 1);
        }
    }
    bits
}

#[cfg(test)]
witness!(keccak256256test);
#[cfg(test)]
witness!(multiplier2);

#[test]
fn build_keccak_witness() {
    let input_vec = vec![
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let bits = bytes_to_bits(&input_vec);
    let big_int_bits = bits
        .into_iter()
        .map(|bit| BigInt::from(bit as u8))
        .collect();
    let mut inputs = HashMap::new();
    inputs.insert("in".to_string(), big_int_bits);

    let now = Instant::now();

    let _out = keccak256256test_witness(inputs);

    // TODO: verify the output

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[test]
fn build_multiplier2_witness() {
    let mut inputs = HashMap::new();
    inputs.insert("a".to_string(), vec![BigInt::from(3)]);
    inputs.insert("b".to_string(), vec![BigInt::from(11)]);

    let now = Instant::now();

    let out = multiplier2_witness(inputs);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // For the multiplier2 circuit we input a = 3 and b = 11 and expect
    // the following witness data
    // 1, 33, 3, 11
    // The first witness entry is always 1. After this there are 3 values
    // defined in the circuit: the two inputs and one output and no intermediates

    assert_eq!(out[0], BigInt::from(1));
    assert_eq!(out[1], BigInt::from(33));
    assert_eq!(out[2], BigInt::from(3));
    assert_eq!(out[3], BigInt::from(11));
}
