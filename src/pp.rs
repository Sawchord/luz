use ark_bls12_381::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{
    hashing::{curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve},
    AffineRepr,
};
use ark_ff::{field_hashers::DefaultFieldHasher, One};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use sha2::Sha256;

pub(crate) struct Pp;

#[derive(Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub(crate) struct Domains {
    pub(crate) h: Radix2EvaluationDomain<Fr>,
    pub(crate) l: Radix2EvaluationDomain<Fr>,
}

impl Pp {
    pub(crate) fn g1() -> G1Affine {
        G1Affine::generator()
    }

    pub(crate) fn g2() -> G2Affine {
        G2Affine::generator()
    }

    pub(crate) fn h1() -> G1Affine {
        let hasher: MapToCurveBasedHasher<G1Projective, DefaultFieldHasher<Sha256>, WBMap<_>> =
            MapToCurveBasedHasher::new(b"h_generator").unwrap();
        hasher.hash(&[]).unwrap()
    }

    pub(crate) fn h2() -> G2Affine {
        let hasher: MapToCurveBasedHasher<G2Projective, DefaultFieldHasher<Sha256>, WBMap<_>> =
            MapToCurveBasedHasher::new(b"h_generator").unwrap();
        hasher.hash(&[]).unwrap()
    }

    pub(crate) fn v1() -> G1Affine {
        let hasher: MapToCurveBasedHasher<G1Projective, DefaultFieldHasher<Sha256>, WBMap<_>> =
            MapToCurveBasedHasher::new(b"v_generator").unwrap();
        hasher.hash(&[]).unwrap()
    }

    pub(crate) fn v2() -> G2Affine {
        let hasher: MapToCurveBasedHasher<G2Projective, DefaultFieldHasher<Sha256>, WBMap<_>> =
            MapToCurveBasedHasher::new(b"v_generator").unwrap();
        hasher.hash(&[]).unwrap()
    }

    pub(crate) fn domains(size_power: u32) -> Domains {
        let size = 2usize.pow(size_power);

        let h_domain: Radix2EvaluationDomain<Fr> = Radix2EvaluationDomain::new(size).unwrap();
        let l_domain = h_domain.get_coset(Fr::one()).unwrap();

        Domains {
            h: h_domain,
            l: l_domain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_smoke() {
        let _domains = Pp::domains(3);
    }
}
