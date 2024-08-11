//! Implementation of the basic signature scheme used in the protocol

use ark_bls12_381::{Fr, G1Affine};
use ark_ec::AffineRepr;
use ark_ff::UniformRand;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use rand::{CryptoRng, RngCore};
use zeroize::Zeroize;

#[derive(Zeroize, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct SecretKey(Fr);

#[derive(Zeroize, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct PublicKey(G1Affine);

// TODO: Proof of possession
// TODO: Sign
// TODO: Verify

impl SecretKey {
    pub fn generate<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self(Fr::rand(rng))
    }

    pub fn pub_key(&self) -> PublicKey {
        PublicKey((G1Affine::generator() * self.0).into())
    }
}
