use crate::{ExtCrs, PublicKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[derive(Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ThresholdSigInstance {
    pub(crate) signers: Vec<(PublicKey, u64)>,
    pub(crate) crs: ExtCrs,
}

impl ThresholdSigInstance {
    pub fn new(signers: Vec<(PublicKey, u64)>, crs: ExtCrs) -> Self {
        Self { signers, crs }
    }
}

// TODO Verification Key
