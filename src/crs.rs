use crate::pp::{Domains, Pp};
use ark_bls12_381::{Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::CurveGroup;
use ark_ff::{Field, UniformRand};
use ark_poly::EvaluationDomain;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use rand::{CryptoRng, RngCore};

#[derive(Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Crs {
    size_power: u32,
    g1_pot: Vec<G1Affine>,
    h1_pot: Vec<G1Affine>,
    g2_tau: G2Affine,
    h2_tau: G2Affine,
}

impl Crs {
    pub fn new_centralized<R: RngCore + CryptoRng>(size_power: u32, rng: &mut R) -> Self {
        let size = 2u64.pow(size_power);

        let tau = Fr::rand(rng);
        let g1 = Pp::g1();
        let h1 = Pp::h1();

        let (g1_pot, h1_pot) = (0..size)
            .map(|i| tau.pow([i]))
            .map(|pot| ((g1 * pot).into_affine(), (h1 * pot).into_affine()))
            .unzip();

        Self {
            size_power,
            g1_pot,
            h1_pot,
            g2_tau: (Pp::g2() * tau).into(),
            h2_tau: (Pp::h2() * tau).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ExtCrs {
    crs: Crs,
    lag: LagrangedKZG,
    g1_eta: G1Affine,
    domains: Domains,
}

#[derive(Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
struct LagrangedKZG {
    g1_h: Vec<G1Affine>,
    h1_h: Vec<G1Affine>,
    g1_l: Vec<G1Affine>,
}

impl ExtCrs {
    pub fn from_crs(crs: Crs) -> Self {
        let size = 2usize.pow(crs.size_power);
        let domains = Pp::domains(crs.size_power);

        let g1_h = lagrange_in_exponent(&crs.g1_pot, &domains.h);
        let h1_h = lagrange_in_exponent(&crs.h1_pot, &domains.h);
        let g1_l = lagrange_in_exponent(&crs.g1_pot, &domains.l);

        let g1_eta = compute_eta_in_exponent(&g1_h, &domains.h);

        Self {
            crs,
            lag: LagrangedKZG { g1_h, h1_h, g1_l },
            g1_eta,
            domains,
        }
    }
}

// TODO: Compute ExtCrs in O(n log n)

fn lagrange_in_exponent<Domain: EvaluationDomain<Fr>>(
    g_pot: &[G1Affine],
    domain: &Domain,
) -> Vec<G1Affine> {
    domain
        .elements()
        .map(|elem| {
            let h_coeffs: Vec<Fr> = domain.evaluate_all_lagrange_coefficients(elem);
            h_coeffs
                .iter()
                .zip(g_pot.iter())
                .map(|(h_coeff, g_tau)| *g_tau * h_coeff)
                .sum::<G1Projective>()
                .into()
        })
        .collect()
}

fn compute_eta_in_exponent<Domain: EvaluationDomain<Fr>>(
    g1_h: &[G1Affine],
    domain: &Domain,
) -> G1Affine {
    domain
        .elements()
        .zip(g1_h.iter())
        .map(|(omega, g1_h)| *g1_h * omega.inverse().unwrap())
        .sum::<G1Projective>()
        .into()
}
