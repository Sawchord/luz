use crate::{errors::AggregationKeyError, pp::Pp, SecretKey, ThresholdSigInstance};
use ark_bls12_381::G1Affine;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use zeroize::Zeroizing;

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct AggregationKeyShare {
    pk: G1Affine,
    // TODO: Include Proof of possession
    g1_h_pk: G1Affine,
    h1_h_pk: G1Affine,
    v_pk: G1Affine,
    g1_eta_pk: G1Affine,
    g1_l_pks: Vec<G1Affine>,
}

impl ThresholdSigInstance {
    pub fn generate_aggregation_key_share(
        &self,
        secret_key: &SecretKey,
    ) -> Result<AggregationKeyShare, AggregationKeyError> {
        // Find out if we are in the instance and what our index is
        let pub_key = secret_key.pub_key();
        let own_index = self
            .signers
            .iter()
            .enumerate()
            .find(|(idx, signer)| signer.0 == pub_key)
            .map(|(idx, _)| idx)
            .ok_or(AggregationKeyError::NotSigner(pub_key))?;

        let pub_key = secret_key.pub_key().0;

        // Calculating the lagranged secret keys
        let secret_key = Zeroizing::new(secret_key.0);
        let g1_h_pk = (self.crs.lag.g1_h[own_index] * *secret_key).into();
        let h1_h_pk = (self.crs.lag.h1_h[own_index] * *secret_key).into();

        let v_pk = (Pp::v1() * *secret_key).into();
        let g1_eta_pk = (self.crs.g1_eta * *secret_key).into();

        let g1_l_pks = (0..self.signers.len())
            .zip(self.crs.lag.g1_l.iter())
            .map(|(_, g1_l)| (*g1_l * *secret_key).into())
            .collect();

        Ok(AggregationKeyShare {
            pk: pub_key,
            g1_h_pk,
            h1_h_pk,
            v_pk,
            g1_eta_pk,
            g1_l_pks,
        })
    }
}

// TODO: Aggregate KeyShares

// TODO: Verify AggregationKeyShare against CRS
// TODO: Verify AggregationKey against CRS
