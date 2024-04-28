//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

pub mod kzg;

pub mod utils;
use kzg::KZG;
use ndarray::{Array, Array2};
use rand::distributions::{Distribution, Uniform};
use sp1_multiProof_program::{linear_forward_activation, ActivationCache, LinearCache};

use std::ops::Mul;
use ark_std::UniformRand;
use ark_bls12_381::{Bls12_381, Fr, G1Projective as G1, G2Projective as G2};
use rand::seq::IteratorRandom;
use ark_ff::Field;
use ark_ec::pairing::Pairing;

use std::collections::HashMap;
use std::path::PathBuf;

struct DeepNeuralNetwork{
    pub layers: Vec<usize>,
    pub learning_rate: f32,
}

impl DeepNeuralNetwork {
    /// Initializes the parameters of the neural network.
    ///
    /// ### Returns
    /// a Hashmap dictionary of randomly initialized weights and biases.
    pub fn initialize_parameters(&self) -> HashMap<String, Array2<f32>> {
        let between = Uniform::from(-1.0..1.0); // random number between -1 and 1
        let mut rng = rand::thread_rng(); // random number generator

        let number_of_layers = self.layers.len();

        let mut parameters: HashMap<String, Array2<f32>> = HashMap::new();

        // start the loop from the first hidden layer to the output layer. 
        // We are not starting from 0 because the zeroth layer is the input layer.
        for l in 1..number_of_layers {
            let weight_array: Vec<f32> = (0..self.layers[l]*self.layers[l-1])
                .map(|_| between.sample(&mut rng))
                .collect(); //create a flattened weights array of (N * M) values

            let bias_array: Vec<f32> = (0..self.layers[l]).map(|_| 0.0).collect();

            let weight_matrix = Array::from_shape_vec((self.layers[l], self.layers[l - 1]), weight_array).unwrap();
            let bias_matrix = Array::from_shape_vec((self.layers[l], 1), bias_array).unwrap();

            let weight_string = ["W", &l.to_string()].join("").to_string();
            let biases_string = ["b", &l.to_string()].join("").to_string();

            parameters.insert(weight_string, weight_matrix);
            parameters.insert(biases_string, bias_matrix);
        }
        parameters
    }

    pub fn forward(
        &self,
        x: &Array2<f32>,
        parameters: &HashMap<String, Array2<f32>>,
    ) -> (Array2<f32>, HashMap<String, (LinearCache, ActivationCache)>) {
        let number_of_layers = self.layers.len()-1;

        let mut a = x.clone();
        let mut caches = HashMap::new();

        for l in 1..number_of_layers {
            let w_string = ["W", &l.to_string()].join("").to_string();
            let b_string = ["b", &l.to_string()].join("").to_string();

            let w = &parameters[&w_string];
            let b = &parameters[&b_string];

            let (a_temp, cache_temp) = linear_forward_activation(&a, w, b, "relu").unwrap();

            a = a_temp;

            caches.insert(l.to_string(), cache_temp);
        }
// Compute activation of last layer with sigmoid
           let weight_string = ["W", &(number_of_layers).to_string()].join("").to_string();
        let bias_string = ["b", &(number_of_layers).to_string()].join("").to_string();

        let w = &parameters[&weight_string];
        let b = &parameters[&bias_string];

        let (al, cache) = linear_forward_activation(&a, w, b, "sigmoid").unwrap();
        caches.insert(number_of_layers.to_string(), cache);


        return (al, caches);
    }
}

pub fn main() {
    // NOTE:  values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    println!("Inside main");

    // let 
    println!("cycle-tracker-start: loading");

    let dnn = DeepNeuralNetwork {
        layers: vec![3,10, 10, 10, 1],
        learning_rate: 0.01,
    };
    let x = Array::from_shape_vec((3, 1), vec![0.1, 0.2, 0.7]).unwrap();
    let parameters = dnn.initialize_parameters();
    let (al, caches) = dnn.forward(&x, &parameters);
    println!("Output: {:?}", al);
    // let n = sp1_zkvm::io::read();
    // let kzg_instance = sp1_zkvm::io::read();

    // let mut rng = ark_std::test_rng();
    // let degree = 16;
    // let mut kzg_instance = KZG::<Bls12_381>::new(
    //     G1::rand(&mut rng),
    //     G2::rand(&mut rng),
    //     degree
    // );

    // // trusted setup ceremony
    // let secret = Fr::rand(&mut rng);
    // kzg_instance.setup(secret);

    // // generate a random polynomial and commit it
    // let poly = vec![Fr::rand(&mut rng); degree+1];
    // let commitment = kzg_instance.commit(&poly);

    // // generate three random points and open the polynomial at those points
    // let points: Vec<Fr> = (0..10).map(|_| Fr::rand(&mut rng)).collect();
    // let pi = kzg_instance.multi_open(&poly, &points);

    // // evaluate the polynomial at those points
    // let mut values = vec![];
    // for i in 0..points.len() {
    //     values.push(utils::evaluate(&poly, points[i]));
    // }

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