use lib::{SerdeSerializableCommitment, SerdeSerializableLagrangePolynomial, SerdeSerializablePoints, SerdeSerializablePolynomial, SerializableG2Commitment, SerializableLagrangePolynomial, SerializablePoints, SerializablePolynomial};
// use sha2::{Digest, Sha256};
use sp1_sdk::{utils, ProverClient, PublicValues, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

use std::ops::Mul;
use ark_std::UniformRand;
use ark_bls12_381::{Bls12_381, Fr, G1Projective as G1, G2Projective as G2};
use rand::seq::IteratorRandom;

use ark_ec::pairing::Pairing;
use ark_ff::{Field, PrimeField};
use ark_std::log2;

// use  utils::{div, mul, evaluate, interpolate};


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

    pub fn setup(&mut self, secret: E::ScalarField) {
        for i in 0..self.degree+1 {
            self.crs_g1.push(self.g1.mul(secret.pow(&[i as u64])));
            self.crs_g2.push(self.g2.mul(secret.pow(&[i as u64])));
        }
        self.g2_tau = self.g2.mul(secret);
    }

    pub fn commit(&self, poly: &[E::ScalarField]) -> E::G1 {
        let mut commitment = self.g1.mul(E::ScalarField::ZERO);
        for i in 0..self.degree+1 {
            commitment += self.crs_g1[i] * poly[i];
        }
        commitment
    }

    pub fn open(&self, poly: &[E::ScalarField], point: E::ScalarField) -> E::G1 {
        // evaluate the polynomial at point
        let value = evaluate(poly, point);

        // initialize denominator
        let denominator = [-point, E::ScalarField::ONE];

        // initialize numerator
        let first = poly[0] - value;
        let rest = &poly[1..];
        let temp: Vec<E::ScalarField> = std::iter::once(first).chain(rest.iter().cloned()).collect();
        let numerator: &[E::ScalarField] = &temp;

        // get quotient by dividing numerator by denominator
        let quotient = div(numerator, &denominator).unwrap();

        // calculate pi as proof (quotient multiplied by CRS)
        let mut pi = self.g1.mul(E::ScalarField::ZERO);
        for i in 0..quotient.len() {
            pi += self.crs_g1[i] * quotient[i];
        }

        // return pi
        pi
    }

    pub fn get_lagrange(&self, poly: &[E::ScalarField], points: &[E::ScalarField]) -> Vec<E::ScalarField>{
        let mut values = vec![];
        for i in 0..points.len() {
            values.push(evaluate(poly, points[i]));
        }
        let mut lagrange_poly = interpolate(points, &values).unwrap();
        lagrange_poly.resize(poly.len(), E::ScalarField::ZERO); // pad with zeros
        lagrange_poly
    }

    pub fn multi_open(&self, poly: &[E::ScalarField], points: &[E::ScalarField]) -> E::G1 {
        // denominator is a polynomial where all its root are points to be evaluated (zero poly)
        let mut zero_poly = vec![-points[0], E::ScalarField::ONE];
        for i in 1..points.len() {
            zero_poly = mul(&zero_poly, &[-points[i], E::ScalarField::ONE]);
        }

        // perform Lagrange interpolation on points
        let mut values = vec![];
        for i in 0..points.len() {
            values.push(evaluate(poly, points[i]));
        }
        let mut lagrange_poly = interpolate(points, &values).unwrap();
        lagrange_poly.resize(poly.len(), E::ScalarField::ZERO); // pad with zeros

        // numerator is the difference between the polynomial and the Lagrange interpolation
        let mut numerator = Vec::with_capacity(poly.len());
        for (coeff1, coeff2) in poly.iter().zip(lagrange_poly.as_slice()) {
            numerator.push(*coeff1 - coeff2);
        }

        // get quotient by dividing numerator by denominator
        let quotient = div(&numerator, &zero_poly).unwrap();

        // calculate pi as proof (quotient multiplied by CRS)
        let mut pi = self.g1.mul(E::ScalarField::ZERO);
        for i in 0..quotient.len() {
            pi += self.crs_g1[i] * quotient[i];
        }

        // return pi
        pi
    }
    

    pub fn verify(
        &self,
        point: E::ScalarField,
        value: E::ScalarField,
        commitment: E::G1,
        pi: E::G1
    ) -> bool {
        let lhs = E::pairing(pi, self.g2_tau - self.g2.mul(point));
        let rhs = E::pairing(commitment - self.g1.mul(value), self.g2);
        lhs == rhs
    }

    pub fn verify_multi(
        &self,
        points: &[E::ScalarField],
        values: &[E::ScalarField],
        commitment: E::G1,
        pi: E::G1
    ) -> bool {
        // compute the zero polynomial
        let mut zero_poly = vec![-points[0], E::ScalarField::ONE];
        for i in 1..points.len() {
            zero_poly = mul(&zero_poly, &[-points[i], E::ScalarField::ONE]);
        }

        // compute commitment of zero polynomial in regards to crs_g2
        let mut zero_commitment = self.g2.mul(E::ScalarField::ZERO);
        for i in 0..zero_poly.len() {
            zero_commitment += self.crs_g2[i] * zero_poly[i];
        }

        // compute lagrange polynomial
        let lagrange_poly = interpolate(points, &values).unwrap();

        // compute commitment of lagrange polynomial in regards to crs_g1
        let mut lagrange_commitment = self.g1.mul(E::ScalarField::ZERO);
        for i in 0..lagrange_poly.len() {
            lagrange_commitment += self.crs_g1[i] * lagrange_poly[i];
        }

        let lhs = E::pairing(pi, zero_commitment);
        let rhs = E::pairing(commitment - lagrange_commitment, self.g2);
        lhs == rhs
    }
}
fn main() {
    // Setup a tracer for logging.
    // utils::setup_tracer();
    // utils::setup_tracer();
    utils::setup_logger();


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

    let points: Vec<Fr> = (0..10).map(|_| Fr::rand(&mut rng)).collect();

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    stdin.write(&SerdeSerializablePoints::from(
        SerializablePoints(points.clone()),
    ));
    stdin.write(&SerdeSerializablePolynomial::from(
        SerializablePolynomial(poly.clone()),
    ));

    
    let mut lagrange_poly: Vec<ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>> = kzg_instance.get_lagrange(&poly, &points);
    stdin.write(&SerdeSerializableLagrangePolynomial::from(
        SerializableLagrangePolynomial(lagrange_poly),
    ));

    let commitment = kzg_instance.commit(&poly);
    stdin.write(&SerdeSerializableCommitment::from(
        SerializableCommitment(commitment),
    ));

    let mut zero_poly = vec![-points[0], E::ScalarField::ONE];
    for i in 1..points.len() {
        zero_poly = mul(&zero_poly, &[-points[i], E::ScalarField::ONE]);
    }

    // compute commitment of zero polynomial in regards to crs_g2
    let mut zero_commitment = self.g2.mul(E::ScalarField::ZERO);
    for i in 0..zero_poly.len() {
        zero_commitment += self.crs_g2[i] * zero_poly[i];
    }
    stdin.write(&SerdeSerializableCommitment::from(
        SerializableG2Commitment(zero_commitment),
    ));



    

    


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



// helper function for polynomial addition
pub fn add<E:Field>(p1: &[E], p2: &[E]) -> Vec<E> {
    let mut result = vec![E::ZERO; std::cmp::max(p1.len(), p2.len())];

    for (i, &coeff) in p1.iter().enumerate() {
        result[i] += coeff;
    }
    for (i, &coeff) in p2.iter().enumerate() {
        result[i] += coeff;
    }

    result
}

// helper function for polynomial multiplication
pub fn mul<E:Field>(p1: &[E], p2: &[E]) -> Vec<E> {
    let mut result = vec![E::ZERO; p1.len() + p2.len() - 1];

    for (i, &coeff1) in p1.iter().enumerate() {
        for (j, &coeff2) in p2.iter().enumerate() {
            result[i + j] += coeff1 * coeff2;
        }
    }

    result
}

// helper function for polynomial division
pub fn div<E:Field>(p1: &[E], p2: &[E]) -> Result<Vec<E>, &'static str> {
    if p2.is_empty() || p2.iter().all(|&x| x == E::ZERO) {
        return Err("Cannot divide by zero polynomial");
    }

    if p1.len() < p2.len() {
        return Ok(vec![E::ZERO]);
    }

    let mut quotient = vec![E::ZERO; p1.len() - p2.len() + 1];
    let mut remainder: Vec<E> = p1.to_vec();

    while remainder.len() >= p2.len() {
        let coeff = *remainder.last().unwrap() / *p2.last().unwrap();
        let pos = remainder.len() - p2.len();

        quotient[pos] = coeff;

        for (i, &factor) in p2.iter().enumerate() {
            remainder[pos + i] -= factor * coeff;
        }

        while let Some(true) = remainder.last().map(|x| *x == E::ZERO) {
            remainder.pop();
        }
    }

    Ok(quotient)
}

// helper function to evaluate polynomial at a point
pub fn evaluate<E:Field>(poly: &[E], point: E) -> E {
    let mut value = E::ZERO;

    for i in 0..poly.len() {
        value += poly[i] * point.pow(&[i as u64]);
    }

    value
}

// helper function to perform Lagrange interpolation given a set of points
pub fn interpolate<E:Field>(points: &[E], values: &[E]) -> Result<Vec<E>, &'static str> {
    if points.len() != values.len() {
        return Err("Number of points and values do not match");
    }

    let mut result = vec![E::ZERO; points.len()];

    for i in 0..points.len() {
        let mut numerator = vec![E::ONE];
        let mut denominator = E::ONE;

        for j in 0..points.len() {
            if i == j {
                continue;
            }

            numerator = mul(&numerator, &[-points[j], E::ONE]);
            denominator *= points[i] - points[j];
        }

        let denominator_inv = denominator.inverse().unwrap();
        let term: Vec<E> = numerator.iter().map(|&x| x * values[i] * denominator_inv).collect();

        result = add(&result, &term);
    }

    Ok(result)
}

// helper function to get the roots of unity of a polynomial
pub fn get_omega<E:PrimeField>(coefficients: &[E]) -> E {
    let mut coefficients = coefficients.to_vec();
    let n = coefficients.len() - 1;
    if !n.is_power_of_two() {
        let num_coeffs = coefficients.len().checked_next_power_of_two().unwrap();
        // pad the coefficients with zeros to the nearest power of two
        for i in coefficients.len()..num_coeffs {
            coefficients[i] = E::ZERO;
        }
    }

    let m = coefficients.len();
    let exp = log2(m);
    let mut omega = E::TWO_ADIC_ROOT_OF_UNITY;
    for _ in exp..E::TWO_ADICITY {
        omega.square_in_place();
    }
    omega
}

// helper function to multiple a polynomial with a scalar value
pub fn scalar_mul<E:Field>(poly: &[E], scalar: E) -> Vec<E> {
    let mut result = Vec::with_capacity(poly.len());
    for coeff in poly {
        result.push(*coeff * scalar);
    }
    result    
}

