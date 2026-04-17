//! Constants for the Ycash main network.
//!
//! Values match ycashd's `chainparams.cpp` under `CMainParams`.

/// The mainnet coin type for YEC, as defined by [SLIP 44].
///
/// [SLIP 44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
pub const COIN_TYPE: u32 = 347;

/// The HRP for a Bech32-encoded Ycash mainnet Sapling extended spending key.
///
/// Shares the Zcash value because ZIP 32 did not fork the HRP at Ycash activation.
pub const HRP_SAPLING_EXTENDED_SPENDING_KEY: &str = "secret-extended-key-main";

/// The HRP for a Bech32-encoded Ycash mainnet Sapling extended full viewing key.
pub const HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY: &str = "zxviews";

/// The HRP for a Bech32-encoded Ycash mainnet Sapling payment address.
///
/// Ycash diverged from Zcash's `zs` at the Ycash activation.
pub const HRP_SAPLING_PAYMENT_ADDRESS: &str = "ys";

/// The prefix for a Base58Check-encoded Ycash mainnet Sprout address.
///
/// Guarantees the first two characters are `yc`.
pub const B58_SPROUT_ADDRESS_PREFIX: [u8; 2] = [0x16, 0x36];

/// The prefix for a Base58Check-encoded Ycash mainnet secret key.
pub const B58_SECRET_KEY_PREFIX: [u8; 1] = [0x80];

/// The prefix for a Base58Check-encoded Ycash mainnet `PublicKeyHash`.
///
/// Guarantees the first two characters are `s1`.
pub const B58_PUBKEY_ADDRESS_PREFIX: [u8; 2] = [0x1c, 0x28];

/// The prefix for a Base58Check-encoded Ycash mainnet `ScriptHash`.
///
/// Guarantees the first two characters are `s3`.
pub const B58_SCRIPT_ADDRESS_PREFIX: [u8; 2] = [0x1c, 0x2c];

/// Sentinel HRP — Ycash does not define a TEX address format.
///
/// A distinct prefix is used so Zcash TEX addresses cannot be misparsed as Ycash.
pub const HRP_TEX_ADDRESS: &str = "ytex";

/// Sentinel HRP — Ycash never activated NU5, so Unified Addresses are unsupported.
///
/// A distinct prefix is used so Zcash UAs cannot be misparsed as Ycash.
pub const HRP_UNIFIED_ADDRESS: &str = "uy";

/// Sentinel HRP — Unified FVKs are not supported on Ycash.
pub const HRP_UNIFIED_FVK: &str = "uviewy";

/// Sentinel HRP — Unified IVKs are not supported on Ycash.
pub const HRP_UNIFIED_IVK: &str = "uivky";
