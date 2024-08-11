use crate::{SecretKey, ThresholdSigInstance};
use ark_bls12_381::G1Affine;

pub struct AggregationKeyShare {
    pk: G1Affine,
    // TODO: Include Proof of possession
    g1_h_pk: G1Affine,
    h1_h_pk: G1Affine,
}

impl ThresholdSigInstance {
    pub fn generate_aggregation_key_share(sk: &SecretKey) -> AggregationKeyShare {
        todo!()
    }
}

// TODO: Verify AggregationKeyShare
