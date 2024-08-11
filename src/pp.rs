use ark_bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{
    hashing::{curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve},
    AffineRepr,
};
use ark_ff::field_hashers::DefaultFieldHasher;
use sha2::Sha256;

pub(crate) struct Pp;

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
}
