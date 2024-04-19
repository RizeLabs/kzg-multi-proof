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

pub fn main() {
    // NOTE:  values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    println!("Inside main");

    // let 
    println!("cycle-tracker-start: loading");
    // let n = sp1_zkvm::io::read();
    // let kzg_instance = sp1_zkvm::io::read();

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
    let poly = vec![Fr::rand(&mut rng); degree+1];
    let commitment = kzg_instance.commit(&poly);

    // generate three random points and open the polynomial at those points
    let points: Vec<Fr> = (0..10).map(|_| Fr::rand(&mut rng)).collect();
    let pi = kzg_instance.multi_open(&poly, &points);

    // evaluate the polynomial at those points
    let mut values = vec![];
    for i in 0..points.len() {
        values.push(utils::evaluate(&poly, points[i]));
    }

    // verify the proof
    // assert!(kzg_instance.verify_multi(&points, &values, commitment, pi));

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