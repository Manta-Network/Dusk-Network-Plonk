//! This example creates a pedersen hash with only one window, as an example
use std::ops::Add;

use dusk_bytes::{ParseHexStr, Serializable};
use dusk_jubjub::{JubJubAffine, JubJubExtended, JubJubScalar};
use rand::{RngCore, SeedableRng, prelude::StdRng};
/// For circuit, we need to constrain bits input to be in [0,1]
fn perdersen_native(gen: JubJubExtended, bits: &[bool]) -> JubJubExtended {
    let mut curr = gen;
    let identity = JubJubExtended::identity();
    bits.iter().fold(identity.clone(), |prev, bit|{
        let result = prev.add(if *bit {&curr} else { &identity});
        curr = curr.double();
        result
    })
}

fn main(){
    let mut rng = StdRng::seed_from_u64(0x12345678);
    let gen = dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(rng.next_u32() as u64);
    let result = perdersen_native(gen, &[false, false, false, true, false, true, false, true]); 
    println!("Native Result: {:?}", result);
}