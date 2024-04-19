// use sha2::{Digest, Sha256};
use sp1_sdk::{utils, ProverClient, PublicValues, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

use std::ops::Mul;
use ark_std::UniformRand;
use ark_bls12_381::{Bls12_381, Fr, G1Projective as G1, G2Projective as G2};
use rand::seq::IteratorRandom;
use ark_ff::Field;
use ark_ec::pairing::Pairing;


pub struct KZG<E: Pairing> {
    pub g1: E::G1,
    pub g2: E::G2,
    pub g2_tau: E::G2,
    pub degree: usize,
    pub crs_g1: Vec<E::G1>,
    pub crs_g2: Vec<E::G2>,
}

impl <E:Pairing> KZG<E> {
    pub fn new(g1: E::G1, g2: E::G2, degree: usize) -> Self {
        Self {
            g1,
            g2,
            g2_tau: g2.mul(E::ScalarField::ZERO),
            degree,
            crs_g1: vec![],
            crs_g2: vec![],
        }
    }

}
fn main() {
    // Setup a tracer for logging.
    utils::setup_tracer();


    let mut rng = ark_std::test_rng();
    let degree = 16;
    let mut kzg_instance = KZG::<Bls12_381>::new(
        G1::rand(&mut rng),
        G2::rand(&mut rng),
        degree
    );

    // trusted setup ceremony
    let secret = Fr::rand(&mut rng);
    // generate a random polynomial and commit it
    let poly = vec![Fr::rand(&mut rng); degree+1];



    // Create an input stream and write '5000' to it.
    let n = 5000u32;

    // The expected result of the fibonacci calculation
    let expected_a = 3867074829u32;
    let expected_b: u32 = 2448710421u32;

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    // stdin.write(&kzg_instance);
    // stdin.write(&poly);
    // stdin.write(&secret);

    // Generate the proof for the given program and input.
    let client = ProverClient::new();
    let mut proof = client.prove(ELF, stdin).unwrap();

    println!("generated proof");

    // Read and verify the output.
    let n: u32 = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();
    // assert_eq!(a, expected_a);
    // assert_eq!(b, expected_b);

    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof and public values
    client.verify(ELF, &proof).expect("verification failed");

    // let mut pv_hasher = Sha256::new();
    // pv_hasher.update(n.to_le_bytes());
    // pv_hasher.update(expected_a.to_le_bytes());
    // pv_hasher.update(expected_b.to_le_bytes());
    // let expected_pv_digest: &[u8] = &pv_hasher.finalize();

    // let public_values_bytes = proof.proof.shard_proofs[0].public_values.clone();
    // let public_values = PublicValues::from_vec(public_values_bytes);
    // assert_eq!(
    //     public_values.commit_digest_bytes().as_slice(),
    //     expected_pv_digest
    // );

    // // Save the proof.
    // proof
    //     .save("proof-with-pis.json")
    //     .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}