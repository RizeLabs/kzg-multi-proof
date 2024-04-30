//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

pub mod kzg;

pub mod utils;
use kzg::KZG;

use std::ops::Mul;
use ark_std::UniformRand;
use ark_bls12_381::{Bls12_381, Fr, G1Projective as G1, G2Projective as G2};
use rand::seq::IteratorRandom;
use ark_ff::Field;
use ark_ec::pairing::Pairing;
use lib::{SerdeSerializableCommitment, SerdeSerializableG2Commitment, SerdeSerializableLagrangePolynomial, SerdeSerializablePoints, SerdeSerializablePolynomial, SerializableCommitment, SerializableG2Commitment, SerializableLagrangePolynomial, SerializablePoints, SerializablePolynomial};


pub fn main() {
    // NOTE:  values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    println!("Inside main");

    // let 
    println!("cycle-tracker-start: loading");
    let n: u32 = sp1_zkvm::io::read();
    let serializable_pvk = sp1_zkvm::io::read::<SerdeSerializablePoints>();
    let pointsss = SerializablePoints::from(serializable_pvk).0;
    let serializable_poly = sp1_zkvm::io::read::<SerdeSerializablePolynomial>();
    let polyyy = SerializablePolynomial::from(serializable_poly).0;
    let serializable_lag_poly = sp1_zkvm::io::read::<SerdeSerializableLagrangePolynomial>();
    let lag_poly = SerializableLagrangePolynomial::from(serializable_lag_poly).0;
    let seriablizable_commitment = sp1_zkvm::io::read::<SerdeSerializableCommitment>();
    let commitmentt = SerializableCommitment::from(seriablizable_commitment).0;
    let serializable_zero_comm = sp1_zkvm::io::read::<SerdeSerializableG2Commitment>();
    let zero_comm = SerializableG2Commitment::from(serializable_zero_comm).0;
    
    // let p2 = sp1_zkvm::io::read();

    let mut rng = ark_std::test_rng();
    let degree = 16;
    let mut kzg_instance = KZG::<Bls12_381>::new(
        G1::rand(&mut rng),
        G2::rand(&mut rng),
        degree
    );

    // trusted setup ceremony
    let secret = Fr::rand(&mut rng);
    kzg_instance.setup(secret);

    // generate a random polynomial and commit it
    let poly = polyyy;
    let commitment = commitmentt;

    // generate three random points and open the polynomial at those points
    let points: Vec<Fr> = pointsss;
    let pi = kzg_instance.multi_open(&poly, &points);

    // evaluate the polynomial at those points
    let mut values = vec![];
    for i in 0..points.len() {
        values.push(utils::evaluate(&poly, points[i]));
    }

    // verify the proof
    assert!(kzg_instance.verify_multi(&points, &values, commitment, pi));

    println!("Multi points evaluation verified!");
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    println!("cycle-tracker-end: loading");
    println!("cycle-tracker-start: verification");
    // for _ in 1..n {
    //     sum = a + b;
    //     a = b;
    //     b = sum;
    // }

    sp1_zkvm::io::commit(&a);
    sp1_zkvm::io::commit(&b);
}

// for 3 points cyplrs = 1,298,386,543 cycles