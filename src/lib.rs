#[macro_use]
extern crate lazy_static;

mod constants;
mod error;
mod pedersen;

use std::str::FromStr;

use ark_ff::{BigInteger, BigInteger256, FpParameters, FromBytes, PrimeField};
use error::Error;
use leftpad::left_pad_char;
use pedersen::geo_pedersen_hash;
use starknet::core::{crypto::pedersen_hash, types::FieldElement};
use starknet_curve::{Fq, Fr};
use wasm_bindgen::prelude::*;

pub fn field_element_from_be_hex(hex: &str) -> FieldElement {
    let decoded = hex::decode(hex.trim_start_matches("0x")).unwrap();

    if decoded.len() > 32 {
        panic!("hex string too long");
    }

    let mut buffer = [0u8; 32];
    buffer[(32 - decoded.len())..].copy_from_slice(&decoded[..]);

    FieldElement::from_bytes_be(&buffer).unwrap()
}

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are little-endian.
/// Returns its 0x-prefixed lowercase hex representation. Output is big-endian.
// https://github.com/geometryresearch/starknet-signatures/blob/722c5987cb96aee80f230a97fed685194c97b7db/packages/prover/src/pedersen.rs
#[wasm_bindgen]
pub fn pedersen(x: &str, y: &str) -> String {
    let fqx = if !x.starts_with("0x") {
        Fq::from_str(x).unwrap()
    } else {
        Fq::from_be_bytes_mod_order(
            &field_element_from_be_hex(&left_pad_char(&x[2..], 64, '0')[0..64]).to_bytes_be(),
        )
    };
    let fqy = if !y.starts_with("0x") {
        Fq::from_str(y).unwrap()
    } else {
        Fq::from_be_bytes_mod_order(
            &field_element_from_be_hex(&left_pad_char(&y[2..], 64, '0')[0..64]).to_bytes_be(),
        )
    };

    let h = geo_pedersen_hash(&fqx, &fqy);

    let mut formatted_hash = String::from("0x");
    formatted_hash.push_str(&h.into_repr().to_string());
    formatted_hash.to_lowercase()
}

pub fn bytes_safe<F: PrimeField<BigInt = BigInteger256>>(
    unchecked_bytes: &Vec<u8>,
) -> Result<BigInteger256, Error> {
    // FromBytes fails if len != 32, anyway we explicitly check for clear err handling
    if unchecked_bytes.len() != 32 {
        return Err(Error::IncorrectLenError);
    }

    let repr = BigInteger256::read(unchecked_bytes.as_slice()).map_err(|_| Error::IOError)?;

    if repr > F::Params::MODULUS {
        return Err(Error::OverflowError);
    }

    Ok(repr)
}

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are little-endian.
/// Returns its 0x-prefixed lowercase hex representation. Output is big-endian.
/// https://github.com/xJonathanLEI/starknet-rs/blob/89a724f00ba6000120b17f68f6da0b4c982eea2f/starknet-crypto/src/pedersen_hash.rs#L19
#[wasm_bindgen]
pub fn starknet_pedersen(x: Vec<u8>, y: Vec<u8>) -> String {
    let x_repr = bytes_safe::<Fr>(&x).unwrap();

    let xx: [u8; 32] = x_repr.to_bytes_be().try_into().unwrap();

    let y_repr = bytes_safe::<Fr>(&y).unwrap();
    let yy: [u8; 32] = y_repr.to_bytes_be().try_into().unwrap();

    let fa = FieldElement::from_bytes_be(&xx).unwrap();
    let fb = FieldElement::from_bytes_be(&yy).unwrap();

    let h = pedersen_hash(&fa, &fb);

    let mut formatted_hash = String::from("0x");
    formatted_hash.push_str(
        &Fq::from_str(&h.to_string())
            .unwrap()
            .into_repr()
            .to_string(),
    );
    formatted_hash.to_lowercase()
}

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are big-endian.
/// Returns its Cairo representation.
/// https://github.com/xJonathanLEI/starknet-rs/blob/89a724f00ba6000120b17f68f6da0b4c982eea2f/starknet-crypto/src/pedersen_hash.rs#L19
#[wasm_bindgen]
pub fn starknet_pedersen_cairo(x: Vec<u8>, y: Vec<u8>) -> String {
    let x_repr = bytes_safe::<Fr>(&x).unwrap();

    let xx: [u8; 32] = x_repr.to_bytes_be().try_into().unwrap();

    let y_repr = bytes_safe::<Fr>(&y).unwrap();
    let yy: [u8; 32] = y_repr.to_bytes_be().try_into().unwrap();

    let fa = FieldElement::from_bytes_be(&xx).unwrap();
    let fb = FieldElement::from_bytes_be(&yy).unwrap();

    let h = pedersen_hash(&fa, &fb);

    h.to_string()
}
