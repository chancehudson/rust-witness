use fnv::FnvHasher;
use num_bigint::{BigInt, BigUint};
use num_traits::{Signed, ToPrimitive, Zero};
use std::hash::Hasher;
use std::time::Instant;
use std::{collections::HashMap, ffi::c_void};

fn main() {}

/// TODO: pass in Hashmap of bigints to build witness
/// provide optional functions for passing raw bits

#[macro_export]
macro_rules! witness {
    ($x: ident) => {
        paste::item! {
            extern "C" {
                pub fn [<$x Instantiate>](i: *mut c_void, resolveImports: *mut c_void);
                pub fn [<$x FreeInstance>](i: *mut c_void);
                pub fn [<$x _getFieldNumLen32>](i: *mut c_void) -> u32;
                pub fn [<$x _getRawPrime>](i: *mut c_void);
                pub fn [<$x _getWitnessSize>](i: *mut c_void) -> u32;
                pub fn [<$x _readSharedRWMemory>](i: *mut c_void, l0: u32) -> u32;
                pub fn [<$x _writeSharedRWMemory>](i: *mut c_void, l0: u32, l1: u32);
                pub fn [<$x _setInputSignal>](i: *mut c_void, l0: u32, l1: u32, l2: u32);
                pub fn [<$x _getWitness>](i: *mut c_void, l0: u32);
                pub fn [<$x _init>](i: *mut c_void, l0: u32);
            }
        }
        paste::item! {
            pub fn [<$x _witness>]<I: IntoIterator<Item = (String, Vec<BigInt>)>>(inputs: I) -> Vec<BigInt> {
                unsafe {
                    let instance = init();
                    let resolver = resolver();
                    // instantiate the memory structures

                    [<$x Instantiate>](instance, resolver);

                    // ready to build the witness

                    let n32 = [<$x _getFieldNumLen32>](instance);
                    [<$x _getRawPrime>](instance);
                    let mut arr = vec![0; n32 as usize];
                    for x in 0..n32 {
                        let res = [<$x _readSharedRWMemory>](instance, x);
                        arr[(n32 as usize) - (x as usize) - 1] = res;
                    }
                    let prime = from_array32(arr);
                    // let n64 = ((prime.bits() - 1) / 64 + 1) as u32;

                    // prepare for building the witness
                    [<$x _init>](instance, 0);

                    // allocate the inputs
                    for (name, values) in inputs.into_iter() {
                        let (msb, lsb) = fnv(&name);

                        for (i, value) in values.into_iter().enumerate() {
                            let f_arr = to_array32(&value, n32 as usize);
                            for j in 0..n32 {
                                [<$x _writeSharedRWMemory>](
                                    instance,
                                    j,
                                    f_arr[(n32 as usize) - 1 - (j as usize)],
                                );
                            }
                            [<$x _setInputSignal>](instance, msb, lsb, i as u32);
                        }
                    }

                    let mut w = Vec::new();

                    let witness_size = [<$x _getWitnessSize>](instance);
                    for i in 0..witness_size {
                        [<$x _getWitness>](instance, i);
                        let mut arr = vec![0; n32 as usize];
                        for j in 0..n32 {
                            arr[(n32 as usize) - 1 - (j as usize)] =
                                [<$x _readSharedRWMemory>](instance, j);
                        }
                        w.push(from_array32(arr));
                    }

                    // cleanup the c memory
                    [<$x FreeInstance>](instance);
                    cleanup(instance);

                    w

                    // If the witness program produces negative values or values above the prime we should
                    // bring the values into range like below

                    // // convert it to field elements
                    // w.into_iter()
                    //     .map(|w| {
                    //         let w = if w.sign() == num_bigint::Sign::Minus {
                    //             // Need to negate the witness element if negative
                    //             prime.to_biguint().unwrap() - w.abs().to_biguint().unwrap()
                    //         } else {
                    //             w.to_biguint().unwrap()
                    //         };
                    //         w
                    //     })
                    //     .collect::<Vec<_>>()
                }
            }
        }
    };
}

// used for keying the values to signals
pub(crate) fn fnv(inp: &str) -> (u32, u32) {
    let mut hasher = FnvHasher::default();
    hasher.write(inp.as_bytes());
    let h = hasher.finish();

    ((h >> 32) as u32, h as u32)
}

pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::new();
    for &byte in bytes {
        for j in 0..8 {
            let bit = (byte >> j) & 1;
            bits.push(bit == 1);
        }
    }
    bits
}

fn from_array32(arr: Vec<u32>) -> BigInt {
    let mut res = BigInt::zero();
    let radix = BigInt::from(0x100000000u64);
    for &val in arr.iter() {
        res = res * &radix + BigInt::from(val);
    }
    res
}

fn to_array32(s: &BigInt, size: usize) -> Vec<u32> {
    let mut res = vec![0; size];
    let mut rem = s.clone();
    let radix = BigInt::from(0x100000000u64);
    let mut c = size;
    while !rem.is_zero() {
        c -= 1;
        res[c] = (&rem % &radix).to_u32().unwrap();
        rem /= &radix;
    }

    res
}

// shared global functions
extern "C" {
    pub fn init() -> *mut c_void;
    pub fn resolver() -> *mut c_void;
    pub fn cleanup(instance: *mut c_void);
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

    let out = keccak256256test_witness(inputs);

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
