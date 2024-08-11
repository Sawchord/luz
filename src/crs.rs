use ark_bls12_381::{Fr, G1Affine, G2Affine};
use ark_ec::CurveGroup;
use ark_ff::{Field, UniformRand};
use rand::{CryptoRng, RngCore};

use crate::pp::Pp;

pub struct Crs {
    g1_pot: Vec<G1Affine>,
    h1_pot: Vec<G1Affine>,
    g2_tau: G2Affine,
    h2_tau: G2Affine,
}

impl Crs {
    pub fn new_centralized<R: RngCore + CryptoRng>(size: u64, rng: &mut R) -> Self {
        let tau = Fr::rand(rng);
        let g1 = Pp::g1();
        let h1 = Pp::h1();

        let (g1_pot, h1_pot) = (0..size)
            .map(|i| tau.pow([i]))
            .map(|pot| ((g1 * pot).into_affine(), (h1 * pot).into_affine()))
            .unzip();

        Self {
            g1_pot,
            h1_pot,
            g2_tau: (Pp::g2() * tau).into(),
            h2_tau: (Pp::h2() * tau).into(),
        }
    }
}
