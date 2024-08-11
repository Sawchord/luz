use crate::{ExtCrs, PublicKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[derive(Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ThresholdSigInstance {
    signers: Vec<(PublicKey, u64)>,
    crs: ExtCrs,
}

impl ThresholdSigInstance {
    pub fn new(signers: Vec<(PublicKey, u64)>, crs: ExtCrs) -> Self {
        Self { signers, crs }
    }
}
