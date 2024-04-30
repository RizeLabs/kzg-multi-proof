use ark_ec::{pairing::Pairing, short_weierstrass::Projective};
use ark_ff::MontBackend;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Valid, Validate,
};
use ark_bls12_381::{Bls12_381, Config, Fr, FrConfig, G1Projective as G1, G2Projective as G2};

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug)]
pub struct SerializablePoints(pub Vec<Fr>);
impl Valid for SerializablePoints {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalSerialize for SerializablePoints {
    fn serialize_with_mode<W: std::io::Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for SerializablePoints {
    fn deserialize_with_mode<R: std::io::Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        Ok(SerializablePoints(Vec::deserialize_with_mode(
            reader, compress, validate,
        )?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeSerializablePoints(#[serde(with = "serde_bytes")] pub Vec<u8>);
impl From<SerializablePoints> for SerdeSerializablePoints {
    fn from(points: SerializablePoints) -> Self {
        let mut serialized_data = Vec::new();
        points
            .serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        SerdeSerializablePoints(serialized_data)
    }
}

impl From<SerdeSerializablePoints> for SerializablePoints {
    fn from(points: SerdeSerializablePoints) -> Self {
        SerializablePoints::deserialize_uncompressed(&mut &points.0[..])
            .expect("Deserialization failed")
    }
}


#[derive(Clone, Debug)]
pub struct SerializablePolynomial(pub Vec<ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>> );
impl Valid for SerializablePolynomial {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalSerialize for SerializablePolynomial {
    fn serialize_with_mode<W: std::io::Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for SerializablePolynomial {
    fn deserialize_with_mode<R: std::io::Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        Ok(SerializablePolynomial(Vec::deserialize_with_mode(
            reader, compress, validate,
        )?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeSerializablePolynomial(#[serde(with = "serde_bytes")] pub Vec<u8>);
impl From<SerializablePolynomial> for SerdeSerializablePolynomial {
    fn from(points: SerializablePolynomial) -> Self {
        let mut serialized_data = Vec::new();
        points
            .serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        SerdeSerializablePolynomial(serialized_data)
    }
}

impl From<SerdeSerializablePolynomial> for SerializablePolynomial {
    fn from(points: SerdeSerializablePolynomial) -> Self {
        SerializablePolynomial::deserialize_uncompressed(&mut &points.0[..])
            .expect("Deserialization failed")
    }
}


#[derive(Clone, Debug)]
pub struct SerializableLagrangePolynomial(pub Vec<<Bls12_381 as Pairing>::ScalarField>);
// Vec<ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>>
impl Valid for SerializableLagrangePolynomial {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalSerialize for SerializableLagrangePolynomial {
    fn serialize_with_mode<W: std::io::Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for SerializableLagrangePolynomial {
    fn deserialize_with_mode<R: std::io::Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        Ok(SerializableLagrangePolynomial(Vec::deserialize_with_mode(
            reader, compress, validate,
        )?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeSerializableLagrangePolynomial(#[serde(with = "serde_bytes")] pub Vec<u8>);
impl From<SerializableLagrangePolynomial> for SerdeSerializableLagrangePolynomial {
    fn from(points: SerializableLagrangePolynomial) -> Self {
        let mut serialized_data = Vec::new();
        points
            .serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        SerdeSerializableLagrangePolynomial(serialized_data)
    }
}

impl From<SerdeSerializableLagrangePolynomial> for SerializableLagrangePolynomial {
    fn from(points: SerdeSerializableLagrangePolynomial) -> Self {
        SerializableLagrangePolynomial::deserialize_uncompressed(&mut &points.0[..])
            .expect("Deserialization failed")
    }
}

#[derive(Clone, Debug)]
pub struct SerializableCommitment(pub ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>);
impl Valid for SerializableCommitment {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalSerialize for SerializableCommitment {
    fn serialize_with_mode<W: std::io::Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for SerializableCommitment {
    fn deserialize_with_mode<R: std::io::Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        Ok(SerializableCommitment(ark_ec::short_weierstrass::Projective::<ark_bls12_381::g1::Config>::deserialize_with_mode(
            reader, compress, validate,
        )?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeSerializableCommitment(#[serde(with = "serde_bytes")] pub Vec<u8>);
impl From<SerializableCommitment> for SerdeSerializableCommitment {
    fn from(points: SerializableCommitment) -> Self {
        let mut serialized_data = Vec::new();
        points
            .serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        SerdeSerializableCommitment(serialized_data)
    }
}

impl From<SerdeSerializableCommitment> for SerializableCommitment {
    fn from(points: SerdeSerializableCommitment) -> Self {
        SerializableCommitment::deserialize_uncompressed(&mut &points.0[..])
            .expect("Deserialization failed")
    }
}

#[derive(Clone, Debug)]
pub struct SerializableG2Commitment(pub ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>);
impl Valid for SerializableG2Commitment {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalSerialize for SerializableG2Commitment {
    fn serialize_with_mode<W: std::io::Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for SerializableG2Commitment {
    fn deserialize_with_mode<R: std::io::Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        Ok(SerializableG2Commitment(ark_ec::short_weierstrass::Projective::<ark_bls12_381::g2::Config>::deserialize_with_mode(
            reader, compress, validate,
        )?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeSerializableG2Commitment(#[serde(with = "serde_bytes")] pub Vec<u8>);
impl From<SerializableG2Commitment> for SerdeSerializableG2Commitment {
    fn from(points: SerializableG2Commitment) -> Self {
        let mut serialized_data = Vec::new();
        points
            .serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        SerdeSerializableG2Commitment(serialized_data)
    }
}

impl From<SerdeSerializableG2Commitment> for SerializableG2Commitment {
    fn from(points: SerdeSerializableG2Commitment) -> Self {
        SerializableG2Commitment::deserialize_uncompressed(&mut &points.0[..])
            .expect("Deserialization failed")
    }
}







// use ark_bn254::Bn254;
// use ark_ec::pairing::Pairing;
// use ark_groth16::{PreparedVerifyingKey, Proof, ProvingKey};
// use ark_serialize::{
//     CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Valid, Validate,
// };
// use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug)]
// pub struct SerializableProvingKey(pub ProvingKey<Bn254>);

// impl Valid for SerializableProvingKey {
//     fn check(&self) -> Result<(), SerializationError> {
//         Ok(())
//     }
// }

// impl CanonicalSerialize for SerializableProvingKey {
//     fn serialize_with_mode<W: std::io::Write>(
//         &self,
//         writer: W,
//         compress: Compress,
//     ) -> Result<(), SerializationError> {
//         self.0.serialize_with_mode(writer, compress)
//     }

//     fn serialized_size(&self, compress: Compress) -> usize {
//         self.0.serialized_size(compress)
//     }
// }

// impl CanonicalDeserialize for SerializableProvingKey {
//     fn deserialize_with_mode<R: std::io::Read>(
//         reader: R,
//         compress: Compress,
//         validate: Validate,
//     ) -> Result<Self, SerializationError> {
//         Ok(SerializableProvingKey(ProvingKey::deserialize_with_mode(
//             reader, compress, validate,
//         )?))
//     }
// }

// #[derive(Clone, Debug)]
// pub struct SerializableProof(pub Proof<Bn254>);

// impl Valid for SerializableProof {
//     fn check(&self) -> Result<(), SerializationError> {
//         Ok(())
//     }
// }

// impl CanonicalSerialize for SerializableProof {
//     fn serialize_with_mode<W: std::io::Write>(
//         &self,
//         writer: W,
//         compress: Compress,
//     ) -> Result<(), SerializationError> {
//         self.0.serialize_with_mode(writer, compress)
//     }

//     fn serialized_size(&self, compress: Compress) -> usize {
//         self.0.serialized_size(compress)
//     }
// }

// impl CanonicalDeserialize for SerializableProof {
//     fn deserialize_with_mode<R: std::io::Read>(
//         reader: R,
//         compress: Compress,
//         validate: Validate,
//     ) -> Result<Self, SerializationError> {
//         Ok(SerializableProof(Proof::deserialize_with_mode(
//             reader, compress, validate,
//         )?))
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct SerializableInputs(pub Vec<<Bn254 as Pairing>::ScalarField>);

// impl Valid for SerializableInputs {
//     fn check(&self) -> Result<(), SerializationError> {
//         Ok(())
//     }
// }

// impl CanonicalSerialize for SerializableInputs {
//     fn serialize_with_mode<W: std::io::Write>(
//         &self,
//         writer: W,
//         compress: Compress,
//     ) -> Result<(), SerializationError> {
//         self.0.serialize_with_mode(writer, compress)
//     }

//     fn serialized_size(&self, compress: Compress) -> usize {
//         self.0.serialized_size(compress)
//     }
// }

// impl CanonicalDeserialize for SerializableInputs {
//     fn deserialize_with_mode<R: std::io::Read>(
//         reader: R,
//         compress: Compress,
//         validate: Validate,
//     ) -> Result<Self, SerializationError> {
//         Ok(SerializableInputs(Vec::deserialize_with_mode(
//             reader, compress, validate,
//         )?))
//     }
// }

// #[derive(Clone, Debug)]
// pub struct SerializablePreparedVerifyingKey(pub PreparedVerifyingKey<Bn254>);

// impl Valid for SerializablePreparedVerifyingKey {
//     fn check(&self) -> Result<(), SerializationError> {
//         Ok(())
//     }
// }

// impl CanonicalSerialize for SerializablePreparedVerifyingKey {
//     fn serialize_with_mode<W: std::io::Write>(
//         &self,
//         writer: W,
//         compress: Compress,
//     ) -> Result<(), SerializationError> {
//         self.0.serialize_with_mode(writer, compress)
//     }

//     fn serialized_size(&self, compress: Compress) -> usize {
//         self.0.serialized_size(compress)
//     }
// }

// impl CanonicalDeserialize for SerializablePreparedVerifyingKey {
//     fn deserialize_with_mode<R: std::io::Read>(
//         reader: R,
//         compress: Compress,
//         validate: Validate,
//     ) -> Result<Self, SerializationError> {
//         Ok(SerializablePreparedVerifyingKey(
//             PreparedVerifyingKey::deserialize_with_mode(reader, compress, validate)?,
//         ))
//     }
// }

// #[derive(Serialize, Deserialize)]
// pub struct SerdeSerializableProof(#[serde(with = "serde_bytes")] pub Vec<u8>);

// #[derive(Serialize, Deserialize)]
// pub struct SerdeSerializableInputs(#[serde(with = "serde_bytes")] pub Vec<u8>);

// #[derive(Serialize, Deserialize)]
// pub struct SerdeSerializablePreparedVerifyingKey(#[serde(with = "serde_bytes")] pub Vec<u8>);

// impl From<SerializableProof> for SerdeSerializableProof {
//     fn from(proof: SerializableProof) -> Self {
//         let mut serialized_data = Vec::new();
//         proof
//             .serialize_uncompressed(&mut serialized_data)
//             .expect("Serialization failed");
//         SerdeSerializableProof(serialized_data)
//     }
// }

// impl From<SerdeSerializableProof> for SerializableProof {
//     fn from(proof: SerdeSerializableProof) -> Self {
//         SerializableProof::deserialize_uncompressed(&mut &proof.0[..])
//             .expect("Deserialization failed")
//     }
// }

// impl From<SerializableInputs> for SerdeSerializableInputs {
//     fn from(inputs: SerializableInputs) -> Self {
//         let mut serialized_data = Vec::new();
//         inputs
//             .serialize_uncompressed(&mut serialized_data)
//             .expect("Serialization failed");
//         SerdeSerializableInputs(serialized_data)
//     }
// }

// impl From<SerdeSerializableInputs> for SerializableInputs {
//     fn from(inputs: SerdeSerializableInputs) -> Self {
//         SerializableInputs::deserialize_uncompressed(&mut &inputs.0[..])
//             .expect("Deserialization failed")
//     }
// }

// impl From<SerializablePreparedVerifyingKey> for SerdeSerializablePreparedVerifyingKey {
//     fn from(vk: SerializablePreparedVerifyingKey) -> Self {
//         let mut serialized_data = Vec::new();
//         vk.serialize_uncompressed(&mut serialized_data)
//             .expect("Serialization failed");
//         SerdeSerializablePreparedVerifyingKey(serialized_data)
//     }
// }

// impl From<SerdeSerializablePreparedVerifyingKey> for SerializablePreparedVerifyingKey {
//     fn from(vk: SerdeSerializablePreparedVerifyingKey) -> Self {
//         SerializablePreparedVerifyingKey::deserialize_uncompressed(&mut &vk.0[..])
//             .expect("Deserialization failed")
//     }
// }