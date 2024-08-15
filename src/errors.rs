use thiserror::Error;

use crate::PublicKey;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AggregationKeyError {
    #[error("The key is not a signer")]
    NotSigner(PublicKey),
}
