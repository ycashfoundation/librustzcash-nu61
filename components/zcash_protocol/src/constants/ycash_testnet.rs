//! Constants for the Ycash test network.
//!
//! Values match ycashd's `chainparams.cpp` under `CTestNetParams`.

/// The testnet coin type.
///
/// Matches mainnet (347). hhanh00's `ywallet` branch uses the mainnet coin type on
/// testnet as well, and Ywallet ships against that convention; we preserve it here.
pub const COIN_TYPE: u32 = 347;

/// The HRP for a Bech32-encoded Ycash testnet Sapling extended spending key.
pub const HRP_SAPLING_EXTENDED_SPENDING_KEY: &str = "secret-extended-key-test";

/// The HRP for a Bech32-encoded Ycash testnet Sapling extended full viewing key.
pub const HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY: &str = "zxviewtestsapling";

/// The HRP for a Bech32-encoded Ycash testnet Sapling payment address.
pub const HRP_SAPLING_PAYMENT_ADDRESS: &str = "ytestsapling";

/// The prefix for a Base58Check-encoded Ycash testnet Sprout address.
///
/// Guarantees the first two characters are `yt`.
pub const B58_SPROUT_ADDRESS_PREFIX: [u8; 2] = [0x16, 0x52];

/// The prefix for a Base58Check-encoded Ycash testnet secret key.
pub const B58_SECRET_KEY_PREFIX: [u8; 1] = [0xef];

/// The prefix for a Base58Check-encoded Ycash testnet `PublicKeyHash`.
///
/// Guarantees the first two characters are `sm`.
pub const B58_PUBKEY_ADDRESS_PREFIX: [u8; 2] = [0x1c, 0x95];

/// The prefix for a Base58Check-encoded Ycash testnet `ScriptHash`.
///
/// Guarantees the first two characters are `t2`.
pub const B58_SCRIPT_ADDRESS_PREFIX: [u8; 2] = [0x1c, 0x2a];

/// Sentinel HRP — Ycash does not define a TEX address format.
pub const HRP_TEX_ADDRESS: &str = "ytextest";

/// Sentinel HRP — Ycash never activated NU5, so Unified Addresses are unsupported.
pub const HRP_UNIFIED_ADDRESS: &str = "utestyc";

/// Sentinel HRP — Unified FVKs are not supported on Ycash.
pub const HRP_UNIFIED_FVK: &str = "uviewtestyc";

/// Sentinel HRP — Unified IVKs are not supported on Ycash.
pub const HRP_UNIFIED_IVK: &str = "uivktestyc";
