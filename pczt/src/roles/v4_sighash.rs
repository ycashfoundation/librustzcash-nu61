//! v4 (ZIP-243) sighash support for PCZT.
//!
//! PCZT's [`super::signer::EffectsOnly`] authorization erases Sapling Groth16 proof
//! bytes. ZIP-243 (transaction version 4) hashes those proof bytes into
//! `shielded_spends_hash` and `shielded_outputs_hash`, so we can't compute a v4 sighash
//! against `EffectsOnly`. This module defines a parallel
//! [`SaplingProofsOnly`] / [`WithProofs`] authorization that retains the proofs, and a
//! [`pczt_to_tx_data_v4`] helper that produces a `TransactionData` ready for
//! [`v4_signature_hash`].
//!
//! Ordering invariant: for v4 transactions the [`Prover`] role MUST run before the
//! [`Signer`] / [`IoFinalizer`] roles, because the sighash needs the Groth16 proofs.
//! For v5 (ZIP-244) the proofs are not committed to by the sighash and the ordering is
//! flexible.
//!
//! [`Prover`]: super::prover::Prover
//! [`Signer`]: super::signer::Signer
//! [`IoFinalizer`]: super::io_finalizer::IoFinalizer
//! [`v4_signature_hash`]: zcash_primitives::transaction::sighash_v4::v4_signature_hash

use alloc::vec::Vec;

use ff::PrimeField;

use sapling::bundle::{
    Authorization as SaplingAuthorization, GrothProofBytes, OutputDescription, SpendDescription,
};
use zcash_primitives::transaction::{Authorization, TransactionData, TxVersion};
use zcash_protocol::{
    consensus::BranchId,
    constants::{V4_TX_VERSION, V4_VERSION_GROUP_ID},
    value::ZatBalance,
};

use crate::common::{Global, determine_lock_time};

use super::signer::{Error, GlobalError};

/// Sapling authorization that carries proof bytes but no signatures.
///
/// This is what v4 sighash needs: the proofs are part of the hash, and the spend-auth
/// signatures are precisely what the signer is about to produce.
#[derive(Debug)]
pub struct SaplingProofsOnly;

impl SaplingAuthorization for SaplingProofsOnly {
    type SpendProof = GrothProofBytes;
    type OutputProof = GrothProofBytes;
    type AuthSig = ();
}

/// Top-level transaction authorization for v4 sighash computation.
///
/// `OrchardAuth` is cosmetic — v4 Sapling transactions never carry an Orchard bundle,
/// but `Authorization` requires an `OrchardAuth` slot.
#[derive(Debug)]
pub struct WithProofs;

impl Authorization for WithProofs {
    type TransparentAuth = transparent::bundle::EffectsOnly;
    type SaplingAuth = SaplingProofsOnly;
    type OrchardAuth = orchard::bundle::EffectsOnly;
    #[cfg(zcash_unstable = "zfuture")]
    type TzeAuth = core::convert::Infallible;
}

/// Returns true if the PCZT global encodes a v4 Sapling transaction.
pub fn is_v4(global: &Global) -> bool {
    global.tx_version == V4_TX_VERSION && global.version_group_id == V4_VERSION_GROUP_ID
}

/// Builds a `sapling::Bundle` carrying proof bytes from a PCZT Sapling bundle.
///
/// Returns `None` if the bundle has no spends or outputs. Errors if any spend or output
/// is missing its `zkproof` (the Prover must have run first), or if the bundle's
/// `value_sum` does not fit in a `ZatBalance`.
fn extract_sapling_proofs_bundle(
    pczt_bundle: &sapling::pczt::Bundle,
) -> Result<Option<sapling::Bundle<SaplingProofsOnly, ZatBalance>>, Error> {
    let pczt_spends = pczt_bundle.spends();
    let pczt_outputs = pczt_bundle.outputs();

    if pczt_spends.is_empty() && pczt_outputs.is_empty() {
        return Ok(None);
    }

    let anchor_bytes = pczt_bundle.anchor().to_bytes();
    let anchor = Option::<bls12_381::Scalar>::from(bls12_381::Scalar::from_repr(anchor_bytes))
        .ok_or(Error::SaplingV4InvalidAnchor)?;

    let spends = pczt_spends
        .iter()
        .map(|spend| {
            let zkproof = spend
                .zkproof()
                .ok_or(Error::SaplingV4MissingProof)?;
            Ok(SpendDescription::from_parts(
                spend.cv().clone(),
                anchor,
                *spend.nullifier(),
                *spend.rk(),
                zkproof,
                (),
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let outputs = pczt_outputs
        .iter()
        .map(|output| {
            let zkproof = output
                .zkproof()
                .ok_or(Error::SaplingV4MissingProof)?;
            Ok(OutputDescription::from_parts(
                output.cv().clone(),
                *output.cmu(),
                output.ephemeral_key().clone(),
                *output.enc_ciphertext(),
                *output.out_ciphertext(),
                zkproof,
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let value_balance: ZatBalance = i64::try_from(*pczt_bundle.value_sum())
        .ok()
        .and_then(|v| ZatBalance::try_from(v).ok())
        .ok_or(Error::SaplingV4ValueOutOfRange)?;

    Ok(sapling::Bundle::from_parts(
        spends,
        outputs,
        value_balance,
        SaplingProofsOnly,
    ))
}

/// Builds a v4-shaped `TransactionData` from a PCZT.
///
/// Parallel to [`super::signer::pczt_to_tx_data`], but produces a
/// `TransactionData<WithProofs>` suitable for [`v4_signature_hash`]. Caller MUST have
/// already verified that `global.tx_version` / `global.version_group_id` correspond to v4
/// Sapling.
pub(crate) fn pczt_to_tx_data_v4(
    global: &Global,
    transparent: &transparent::pczt::Bundle,
    sapling: &sapling::pczt::Bundle,
) -> Result<TransactionData<WithProofs>, Error> {
    let consensus_branch_id = BranchId::try_from(global.consensus_branch_id)
        .map_err(|_| Error::Global(GlobalError::UnknownConsensusBranchId))?;

    let transparent_bundle = transparent
        .extract_effects()
        .map_err(Error::TransparentExtract)?;
    let sapling_bundle = extract_sapling_proofs_bundle(sapling)?;

    Ok(TransactionData::from_parts(
        TxVersion::V4,
        consensus_branch_id,
        determine_lock_time(global, transparent.inputs()).ok_or(Error::IncompatibleLockTimes)?,
        global.expiry_height.into(),
        #[cfg(all(
            any(zcash_unstable = "nu7", zcash_unstable = "zfuture"),
            feature = "zip-233"
        ))]
        zcash_protocol::value::Zatoshis::ZERO,
        transparent_bundle,
        None,
        sapling_bundle,
        // v4 transactions never carry an Orchard bundle.
        None,
    ))
}
