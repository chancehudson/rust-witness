use fnv::FnvHasher;
pub use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
pub use paste;
use std::hash::Hasher;

pub mod transpile;

#[macro_export]
macro_rules! witness {
    ($x: ident) => {
        rust_witness::paste::item! {
            mod [<$x _witness_c>] {
                extern "C" {
                    pub fn witness_c_init() -> *mut std::ffi::c_void;
                    pub fn witness_c_resolver() -> *mut std::ffi::c_void;
                    pub fn witness_c_cleanup(instance: *mut std::ffi::c_void);
                }
            }
            extern "C" {
                pub fn [<$x Instantiate>](i: *mut std::ffi::c_void, resolveImports: *mut std::ffi::c_void);
                pub fn [<$x FreeInstance>](i: *mut std::ffi::c_void);
                pub fn [<$x _getFieldNumLen32>](i: *mut std::ffi::c_void) -> u32;
                pub fn [<$x _getRawPrime>](i: *mut std::ffi::c_void);
                pub fn [<$x _getWitnessSize>](i: *mut std::ffi::c_void) -> u32;
                pub fn [<$x _readSharedRWMemory>](i: *mut std::ffi::c_void, l0: u32) -> u32;
                pub fn [<$x _writeSharedRWMemory>](i: *mut std::ffi::c_void, l0: u32, l1: u32);
                pub fn [<$x _setInputSignal>](i: *mut std::ffi::c_void, l0: u32, l1: u32, l2: u32);
                pub fn [<$x _getWitness>](i: *mut std::ffi::c_void, l0: u32);
                pub fn [<$x _init>](i: *mut std::ffi::c_void, l0: u32);
            }

            // Public functions to make the above functions accessible
            // in the crate namespace
            pub fn [<$x _c_init>]() -> *mut std::ffi::c_void {
                unsafe { [<$x _witness_c>]::witness_c_init() }
            }

            pub fn [<$x _c_resolver>]() -> *mut std::ffi::c_void {
                unsafe { [<$x _witness_c>]::witness_c_resolver() }
            }

            pub fn [<$x _c_cleanup>](v: *mut std::ffi::c_void) {
                unsafe {
                    [<$x _witness_c>]::witness_c_cleanup(v);
                }
            }
        }
        rust_witness::paste::item! {
            pub fn [<$x _witness>]<I: IntoIterator<Item = (String, Vec<rust_witness::BigInt>)>>(inputs: I) -> Vec<rust_witness::BigInt> {
                // used for keying the values to signals
                unsafe {
                    let instance = [<$x _c_init>]();
                    let resolver = [<$x _c_resolver>]();
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
                    // let prime = from_array32(arr);
                    // let n64 = ((prime.bits() - 1) / 64 + 1) as u32;

                    // prepare for building the witness
                    [<$x _init>](instance, 0);

                    // allocate the inputs
                    for (name, values) in inputs.into_iter() {
                        let (msb, lsb) = rust_witness::fnv(&name);

                        for (i, value) in values.into_iter().enumerate() {
                            let f_arr = rust_witness::to_array32(&value, n32 as usize);
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
                        w.push(rust_witness::from_array32(arr));
                    }

                    // cleanup the c memory
                    [<$x FreeInstance>](instance);
                    [<$x _c_cleanup>](instance);

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

pub fn fnv(inp: &str) -> (u32, u32) {
    let mut hasher = FnvHasher::default();
    hasher.write(inp.as_bytes());
    let h = hasher.finish();

    ((h >> 32) as u32, h as u32)
}

pub fn from_array32(arr: Vec<u32>) -> BigInt {
    let mut res = BigInt::zero();
    let radix = BigInt::from(0x100000000u64);
    for &val in arr.iter() {
        res = res * &radix + BigInt::from(val);
    }
    res
}

pub fn to_array32(s: &BigInt, size: usize) -> Vec<u32> {
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
