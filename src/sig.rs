//! Implementation of the basic signature scheme used in the protocol

use ark_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{
    hashing::{curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve},
    AffineRepr,
};
use ark_ff::{field_hashers::DefaultFieldHasher, UniformRand};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use rand::{CryptoRng, RngCore};
use sha2::Sha256;
use zeroize::Zeroize;

#[derive(Zeroize, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct SecretKey(Fr);

#[derive(Zeroize, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct PublicKey(G1Affine);

#[derive(Zeroize, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Signature(G2Affine);

// TODO: Proof of possession
// TODO: Verify proof of possesion
// TODO: Verify

impl SecretKey {
    pub fn generate<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        Self(Fr::rand(rng))
    }

    pub fn pub_key(&self) -> PublicKey {
        PublicKey((G1Affine::generator() * self.0).into())
    }

    pub fn sign<M: AsRef<[u8]>>(&self, message: M) -> Signature {
        let hasher: MapToCurveBasedHasher<G2Projective, DefaultFieldHasher<Sha256>, WBMap<_>> =
            MapToCurveBasedHasher::new(b"sign").unwrap();
        let fm: G2Affine = hasher.hash(message.as_ref()).unwrap();
        Signature((fm * self.0).into())
    }
}

impl PublicKey {
    pub fn verify<M: AsRef<[u8]>>(&self, message: M, signature: &Signature) -> bool {
        let hasher: MapToCurveBasedHasher<G2Projective, DefaultFieldHasher<Sha256>, WBMap<_>> =
            MapToCurveBasedHasher::new(b"sign").unwrap();
        let fm: G2Affine = hasher.hash(message.as_ref()).unwrap();

        todo!()
    }
}
