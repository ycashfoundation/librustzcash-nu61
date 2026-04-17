//! Consensus logic and parameters.

use core::cmp::{Ord, Ordering};
use core::convert::TryFrom;
use core::fmt;
use core::ops::{Add, Bound, RangeBounds, Sub};

#[cfg(feature = "std")]
use memuse::DynamicUsage;

use crate::constants::{mainnet, regtest, testnet, ycash_mainnet, ycash_testnet};

/// A wrapper type representing blockchain heights.
///
/// Safe conversion from various integer types, as well as addition and subtraction, are
/// provided.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockHeight(u32);

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(BlockHeight);

/// The height of the genesis block on a network.
pub const H0: BlockHeight = BlockHeight(0);

impl BlockHeight {
    pub const fn from_u32(v: u32) -> BlockHeight {
        BlockHeight(v)
    }

    /// Subtracts the provided value from this height, returning [`H0`] if this would result in
    /// underflow of the wrapped `u32`.
    pub fn saturating_sub(self, v: u32) -> BlockHeight {
        BlockHeight(self.0.saturating_sub(v))
    }
}

impl fmt::Display for BlockHeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Ord for BlockHeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for BlockHeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u32> for BlockHeight {
    fn from(value: u32) -> Self {
        BlockHeight(value)
    }
}

impl From<BlockHeight> for u32 {
    fn from(value: BlockHeight) -> u32 {
        value.0
    }
}

impl TryFrom<u64> for BlockHeight {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

impl From<BlockHeight> for u64 {
    fn from(value: BlockHeight) -> u64 {
        value.0 as u64
    }
}

impl TryFrom<i32> for BlockHeight {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

impl TryFrom<i64> for BlockHeight {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

impl From<BlockHeight> for i64 {
    fn from(value: BlockHeight) -> i64 {
        value.0 as i64
    }
}

impl Add<u32> for BlockHeight {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        BlockHeight(self.0.saturating_add(other))
    }
}

impl Sub<u32> for BlockHeight {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        BlockHeight(self.0.saturating_sub(other))
    }
}

impl Sub<BlockHeight> for BlockHeight {
    type Output = u32;

    fn sub(self, other: BlockHeight) -> u32 {
        self.0.saturating_sub(other.0)
    }
}

/// The enumeration of known Zcash network types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NetworkType {
    /// Zcash Mainnet.
    Main,
    /// Zcash Testnet.
    Test,
    /// Private integration / regression testing, used in `zcashd`.
    ///
    /// For some address types there is no distinction between test and regtest encodings;
    /// those will always be parsed as `Network::Test`.
    Regtest,
    /// Ycash Mainnet.
    YcashMain,
    /// Ycash Testnet.
    YcashTest,
}

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(NetworkType);

pub(crate) mod private {
    pub trait Sealed {}
}

/// Constants associated with a given Zcash network.
pub trait NetworkConstants: private::Sealed + Clone {
    /// The coin type for ZEC, as defined by [SLIP 44].
    ///
    /// [SLIP 44]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    fn coin_type(&self) -> u32;

    /// Returns the human-readable prefix for Bech32-encoded Sapling extended spending keys
    /// for the network to which this NetworkConstants value applies.
    ///
    /// Defined in [ZIP 32].
    ///
    /// [ZIP 32]: https://github.com/zcash/zips/blob/main/zips/zip-0032.rst
    fn hrp_sapling_extended_spending_key(&self) -> &'static str;

    /// Returns the human-readable prefix for Bech32-encoded Sapling extended full
    /// viewing keys for the network to which this NetworkConstants value applies.
    ///
    /// Defined in [ZIP 32].
    ///
    /// [ZIP 32]: https://github.com/zcash/zips/blob/master/zip-0032.rst
    fn hrp_sapling_extended_full_viewing_key(&self) -> &'static str;

    /// Returns the Bech32-encoded human-readable prefix for Sapling payment addresses
    /// for the network to which this NetworkConstants value applies.
    ///
    /// Defined in section 5.6.4 of the [Zcash Protocol Specification].
    ///
    /// [Zcash Protocol Specification]: https://github.com/zcash/zips/blob/main/rendered/protocol/protocol.pdf
    fn hrp_sapling_payment_address(&self) -> &'static str;

    /// Returns the human-readable prefix for Base58Check-encoded Sprout
    /// payment addresses for the network to which this NetworkConstants value
    /// applies.
    ///
    /// Defined in the [Zcash Protocol Specification section 5.6.3][sproutpaymentaddrencoding].
    ///
    /// [sproutpaymentaddrencoding]: https://zips.z.cash/protocol/protocol.pdf#sproutpaymentaddrencoding
    fn b58_sprout_address_prefix(&self) -> [u8; 2];

    /// Returns the human-readable prefix for Base58Check-encoded transparent
    /// pay-to-public-key-hash payment addresses for the network to which this NetworkConstants value
    /// applies.
    fn b58_pubkey_address_prefix(&self) -> [u8; 2];

    /// Returns the human-readable prefix for Base58Check-encoded transparent secret key for the
    /// network to which this NetworkConstants value applies.
    fn b58_secret_key_prefix(&self) -> [u8; 1];

    /// Returns the human-readable prefix for Base58Check-encoded transparent pay-to-script-hash
    /// payment addresses for the network to which this NetworkConstants value applies.
    fn b58_script_address_prefix(&self) -> [u8; 2];

    /// Returns the Bech32-encoded human-readable prefix for TEX addresses, for the
    /// network to which this `NetworkConstants` value applies.
    ///
    /// Defined in [ZIP 320].
    ///
    /// [ZIP 320]: https://zips.z.cash/zip-0320
    fn hrp_tex_address(&self) -> &'static str;

    /// The HRP for a Bech32m-encoded mainnet Unified Address.
    ///
    /// Defined in [ZIP 316][zip-0316].
    ///
    /// [zip-0316]: https://zips.z.cash/zip-0316
    fn hrp_unified_address(&self) -> &'static str;

    /// The HRP for a Bech32m-encoded mainnet Unified FVK.
    ///
    /// Defined in [ZIP 316][zip-0316].
    ///
    /// [zip-0316]: https://zips.z.cash/zip-0316
    fn hrp_unified_fvk(&self) -> &'static str;

    /// The HRP for a Bech32m-encoded mainnet Unified IVK.
    ///
    /// Defined in [ZIP 316][zip-0316].
    ///
    /// [zip-0316]: https://zips.z.cash/zip-0316
    fn hrp_unified_ivk(&self) -> &'static str;
}

impl private::Sealed for NetworkType {}

impl NetworkConstants for NetworkType {
    fn coin_type(&self) -> u32 {
        match self {
            NetworkType::Main => mainnet::COIN_TYPE,
            NetworkType::Test => testnet::COIN_TYPE,
            NetworkType::Regtest => regtest::COIN_TYPE,
            NetworkType::YcashMain => ycash_mainnet::COIN_TYPE,
            NetworkType::YcashTest => ycash_testnet::COIN_TYPE,
        }
    }

    fn hrp_sapling_extended_spending_key(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            NetworkType::Test => testnet::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            NetworkType::Regtest => regtest::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            NetworkType::YcashMain => ycash_mainnet::HRP_SAPLING_EXTENDED_SPENDING_KEY,
            NetworkType::YcashTest => ycash_testnet::HRP_SAPLING_EXTENDED_SPENDING_KEY,
        }
    }

    fn hrp_sapling_extended_full_viewing_key(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY,
            NetworkType::Test => testnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY,
            NetworkType::Regtest => regtest::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY,
            NetworkType::YcashMain => ycash_mainnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY,
            NetworkType::YcashTest => ycash_testnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY,
        }
    }

    fn hrp_sapling_payment_address(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_SAPLING_PAYMENT_ADDRESS,
            NetworkType::Test => testnet::HRP_SAPLING_PAYMENT_ADDRESS,
            NetworkType::Regtest => regtest::HRP_SAPLING_PAYMENT_ADDRESS,
            NetworkType::YcashMain => ycash_mainnet::HRP_SAPLING_PAYMENT_ADDRESS,
            NetworkType::YcashTest => ycash_testnet::HRP_SAPLING_PAYMENT_ADDRESS,
        }
    }

    fn b58_sprout_address_prefix(&self) -> [u8; 2] {
        match self {
            NetworkType::Main => mainnet::B58_SPROUT_ADDRESS_PREFIX,
            NetworkType::Test => testnet::B58_SPROUT_ADDRESS_PREFIX,
            NetworkType::Regtest => regtest::B58_SPROUT_ADDRESS_PREFIX,
            NetworkType::YcashMain => ycash_mainnet::B58_SPROUT_ADDRESS_PREFIX,
            NetworkType::YcashTest => ycash_testnet::B58_SPROUT_ADDRESS_PREFIX,
        }
    }

    fn b58_pubkey_address_prefix(&self) -> [u8; 2] {
        match self {
            NetworkType::Main => mainnet::B58_PUBKEY_ADDRESS_PREFIX,
            NetworkType::Test => testnet::B58_PUBKEY_ADDRESS_PREFIX,
            NetworkType::Regtest => regtest::B58_PUBKEY_ADDRESS_PREFIX,
            NetworkType::YcashMain => ycash_mainnet::B58_PUBKEY_ADDRESS_PREFIX,
            NetworkType::YcashTest => ycash_testnet::B58_PUBKEY_ADDRESS_PREFIX,
        }
    }

    fn b58_secret_key_prefix(&self) -> [u8; 1] {
        match self {
            NetworkType::Main => mainnet::B58_SECRET_KEY_PREFIX,
            NetworkType::Test => testnet::B58_SECRET_KEY_PREFIX,
            NetworkType::Regtest => regtest::B58_SECRET_KEY_PREFIX,
            NetworkType::YcashMain => ycash_mainnet::B58_SECRET_KEY_PREFIX,
            NetworkType::YcashTest => ycash_testnet::B58_SECRET_KEY_PREFIX,
        }
    }

    fn b58_script_address_prefix(&self) -> [u8; 2] {
        match self {
            NetworkType::Main => mainnet::B58_SCRIPT_ADDRESS_PREFIX,
            NetworkType::Test => testnet::B58_SCRIPT_ADDRESS_PREFIX,
            NetworkType::Regtest => regtest::B58_SCRIPT_ADDRESS_PREFIX,
            NetworkType::YcashMain => ycash_mainnet::B58_SCRIPT_ADDRESS_PREFIX,
            NetworkType::YcashTest => ycash_testnet::B58_SCRIPT_ADDRESS_PREFIX,
        }
    }

    fn hrp_tex_address(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_TEX_ADDRESS,
            NetworkType::Test => testnet::HRP_TEX_ADDRESS,
            NetworkType::Regtest => regtest::HRP_TEX_ADDRESS,
            NetworkType::YcashMain => ycash_mainnet::HRP_TEX_ADDRESS,
            NetworkType::YcashTest => ycash_testnet::HRP_TEX_ADDRESS,
        }
    }

    fn hrp_unified_address(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_UNIFIED_ADDRESS,
            NetworkType::Test => testnet::HRP_UNIFIED_ADDRESS,
            NetworkType::Regtest => regtest::HRP_UNIFIED_ADDRESS,
            NetworkType::YcashMain => ycash_mainnet::HRP_UNIFIED_ADDRESS,
            NetworkType::YcashTest => ycash_testnet::HRP_UNIFIED_ADDRESS,
        }
    }

    fn hrp_unified_fvk(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_UNIFIED_FVK,
            NetworkType::Test => testnet::HRP_UNIFIED_FVK,
            NetworkType::Regtest => regtest::HRP_UNIFIED_FVK,
            NetworkType::YcashMain => ycash_mainnet::HRP_UNIFIED_FVK,
            NetworkType::YcashTest => ycash_testnet::HRP_UNIFIED_FVK,
        }
    }

    fn hrp_unified_ivk(&self) -> &'static str {
        match self {
            NetworkType::Main => mainnet::HRP_UNIFIED_IVK,
            NetworkType::Test => testnet::HRP_UNIFIED_IVK,
            NetworkType::Regtest => regtest::HRP_UNIFIED_IVK,
            NetworkType::YcashMain => ycash_mainnet::HRP_UNIFIED_IVK,
            NetworkType::YcashTest => ycash_testnet::HRP_UNIFIED_IVK,
        }
    }
}

/// Zcash consensus parameters.
pub trait Parameters: Clone {
    /// Returns the type of network configured by this set of consensus parameters.
    fn network_type(&self) -> NetworkType;

    /// Returns the activation height for a particular network upgrade,
    /// if an activation height has been set.
    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight>;

    /// Returns the consensus branch ID associated with a network upgrade on this network.
    ///
    /// The default maps each network upgrade to its canonical Zcash branch ID. Networks
    /// that fork Zcash's consensus rules (e.g. Ycash, which uses distinct branch IDs for
    /// Blossom/Heartwood/Canopy after the Ycash fork for two-way replay protection) must
    /// override this to return the appropriate network-specific branch IDs.
    fn branch_id(&self, nu: NetworkUpgrade) -> BranchId {
        nu.branch_id()
    }

    /// Determines whether the specified network upgrade is active as of the
    /// provided block height on the network to which this Parameters value applies.
    fn is_nu_active(&self, nu: NetworkUpgrade, height: BlockHeight) -> bool {
        self.activation_height(nu).is_some_and(|h| h <= height)
    }
}

impl<P: Parameters> Parameters for &P {
    fn network_type(&self) -> NetworkType {
        (*self).network_type()
    }

    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        (*self).activation_height(nu)
    }

    fn branch_id(&self, nu: NetworkUpgrade) -> BranchId {
        (*self).branch_id(nu)
    }
}

impl<P: Parameters> private::Sealed for P {}

impl<P: Parameters> NetworkConstants for P {
    fn coin_type(&self) -> u32 {
        self.network_type().coin_type()
    }

    fn hrp_sapling_extended_spending_key(&self) -> &'static str {
        self.network_type().hrp_sapling_extended_spending_key()
    }

    fn hrp_sapling_extended_full_viewing_key(&self) -> &'static str {
        self.network_type().hrp_sapling_extended_full_viewing_key()
    }

    fn hrp_sapling_payment_address(&self) -> &'static str {
        self.network_type().hrp_sapling_payment_address()
    }

    fn b58_sprout_address_prefix(&self) -> [u8; 2] {
        self.network_type().b58_sprout_address_prefix()
    }

    fn b58_pubkey_address_prefix(&self) -> [u8; 2] {
        self.network_type().b58_pubkey_address_prefix()
    }

    fn b58_secret_key_prefix(&self) -> [u8; 1] {
        self.network_type().b58_secret_key_prefix()
    }

    fn b58_script_address_prefix(&self) -> [u8; 2] {
        self.network_type().b58_script_address_prefix()
    }

    fn hrp_tex_address(&self) -> &'static str {
        self.network_type().hrp_tex_address()
    }

    fn hrp_unified_address(&self) -> &'static str {
        self.network_type().hrp_unified_address()
    }

    fn hrp_unified_fvk(&self) -> &'static str {
        self.network_type().hrp_unified_fvk()
    }

    fn hrp_unified_ivk(&self) -> &'static str {
        self.network_type().hrp_unified_ivk()
    }
}

/// Marker struct for the production network.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct MainNetwork;

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(MainNetwork);

/// The production network.
pub const MAIN_NETWORK: MainNetwork = MainNetwork;

impl Parameters for MainNetwork {
    fn network_type(&self) -> NetworkType {
        NetworkType::Main
    }

    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match nu {
            NetworkUpgrade::Overwinter => Some(BlockHeight(347_500)),
            NetworkUpgrade::Sapling => Some(BlockHeight(419_200)),
            NetworkUpgrade::Ycash => None,
            NetworkUpgrade::Blossom => Some(BlockHeight(653_600)),
            NetworkUpgrade::Heartwood => Some(BlockHeight(903_000)),
            NetworkUpgrade::Canopy => Some(BlockHeight(1_046_400)),
            NetworkUpgrade::Nu5 => Some(BlockHeight(1_687_104)),
            NetworkUpgrade::Nu6 => Some(BlockHeight(2_726_400)),
            NetworkUpgrade::Nu6_1 => Some(BlockHeight(3_146_400)),
            #[cfg(zcash_unstable = "nu7")]
            NetworkUpgrade::Nu7 => None,
            #[cfg(zcash_unstable = "zfuture")]
            NetworkUpgrade::ZFuture => None,
        }
    }
}

/// Marker struct for the test network.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct TestNetwork;

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(TestNetwork);

/// The test network.
pub const TEST_NETWORK: TestNetwork = TestNetwork;

impl Parameters for TestNetwork {
    fn network_type(&self) -> NetworkType {
        NetworkType::Test
    }

    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match nu {
            NetworkUpgrade::Overwinter => Some(BlockHeight(207_500)),
            NetworkUpgrade::Sapling => Some(BlockHeight(280_000)),
            NetworkUpgrade::Ycash => None,
            NetworkUpgrade::Blossom => Some(BlockHeight(584_000)),
            NetworkUpgrade::Heartwood => Some(BlockHeight(903_800)),
            NetworkUpgrade::Canopy => Some(BlockHeight(1_028_500)),
            NetworkUpgrade::Nu5 => Some(BlockHeight(1_842_420)),
            NetworkUpgrade::Nu6 => Some(BlockHeight(2_976_000)),
            NetworkUpgrade::Nu6_1 => Some(BlockHeight(3_536_500)),
            #[cfg(zcash_unstable = "nu7")]
            NetworkUpgrade::Nu7 => None,
            #[cfg(zcash_unstable = "zfuture")]
            NetworkUpgrade::ZFuture => None,
        }
    }
}

/// The enumeration of known Zcash networks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Network {
    /// Zcash Mainnet.
    MainNetwork,
    /// Zcash Testnet.
    TestNetwork,
}

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(Network);

impl Parameters for Network {
    fn network_type(&self) -> NetworkType {
        match self {
            Network::MainNetwork => NetworkType::Main,
            Network::TestNetwork => NetworkType::Test,
        }
    }

    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match self {
            Network::MainNetwork => MAIN_NETWORK.activation_height(nu),
            Network::TestNetwork => TEST_NETWORK.activation_height(nu),
        }
    }
}

mod ycash;

pub use ycash::{MainNetwork as YcashMainNetwork, TestNetwork as YcashTestNetwork};

/// The Ycash production network.
pub const YCASH_MAIN_NETWORK: YcashMainNetwork = YcashMainNetwork;

/// The Ycash test network.
pub const YCASH_TEST_NETWORK: YcashTestNetwork = YcashTestNetwork;

/// An event that occurs at a specified height on the Zcash chain, at which point the
/// consensus rules enforced by the network are altered.
///
/// See [ZIP 200](https://zips.z.cash/zip-0200) for more details.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkUpgrade {
    /// The [Overwinter] network upgrade.
    ///
    /// [Overwinter]: https://z.cash/upgrade/overwinter/
    Overwinter,
    /// The [Sapling] network upgrade.
    ///
    /// [Sapling]: https://z.cash/upgrade/sapling/
    Sapling,
    /// The Ycash chain-fork network upgrade.
    ///
    /// Ycash split from Zcash at this activation (mainnet height 570000, testnet 510248).
    /// On non-Ycash networks this upgrade has no activation height.
    Ycash,
    /// The [Blossom] network upgrade.
    ///
    /// [Blossom]: https://z.cash/upgrade/blossom/
    Blossom,
    /// The [Heartwood] network upgrade.
    ///
    /// [Heartwood]: https://z.cash/upgrade/heartwood/
    Heartwood,
    /// The [Canopy] network upgrade.
    ///
    /// [Canopy]: https://z.cash/upgrade/canopy/
    Canopy,
    /// The [Nu5] network upgrade.
    ///
    /// [Nu5]: https://z.cash/upgrade/nu5/
    Nu5,
    /// The [Nu6] network upgrade.
    ///
    /// [Nu6]: https://z.cash/upgrade/nu6/
    Nu6,
    /// The [Nu6.1] network upgrade.
    ///
    /// [Nu6.1]: https://z.cash/upgrade/nu6.1/
    Nu6_1,
    /// The [Nu7 (proposed)] network upgrade.
    ///
    /// [Nu7 (proposed)]: https://z.cash/upgrade/nu7/
    #[cfg(zcash_unstable = "nu7")]
    Nu7,
    /// The ZFUTURE network upgrade.
    ///
    /// This upgrade is expected never to activate on mainnet;
    /// it is intended for use in integration testing of functionality
    /// that is a candidate for integration in a future network upgrade.
    #[cfg(zcash_unstable = "zfuture")]
    ZFuture,
}

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(NetworkUpgrade);

impl fmt::Display for NetworkUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkUpgrade::Overwinter => write!(f, "Overwinter"),
            NetworkUpgrade::Sapling => write!(f, "Sapling"),
            NetworkUpgrade::Ycash => write!(f, "Ycash"),
            NetworkUpgrade::Blossom => write!(f, "Blossom"),
            NetworkUpgrade::Heartwood => write!(f, "Heartwood"),
            NetworkUpgrade::Canopy => write!(f, "Canopy"),
            NetworkUpgrade::Nu5 => write!(f, "Nu5"),
            NetworkUpgrade::Nu6 => write!(f, "Nu6"),
            NetworkUpgrade::Nu6_1 => write!(f, "Nu6.1"),
            #[cfg(zcash_unstable = "nu7")]
            NetworkUpgrade::Nu7 => write!(f, "Nu7"),
            #[cfg(zcash_unstable = "zfuture")]
            NetworkUpgrade::ZFuture => write!(f, "ZFUTURE"),
        }
    }
}

impl NetworkUpgrade {
    fn branch_id(self) -> BranchId {
        match self {
            NetworkUpgrade::Overwinter => BranchId::Overwinter,
            NetworkUpgrade::Sapling => BranchId::Sapling,
            NetworkUpgrade::Ycash => BranchId::Ycash,
            NetworkUpgrade::Blossom => BranchId::Blossom,
            NetworkUpgrade::Heartwood => BranchId::Heartwood,
            NetworkUpgrade::Canopy => BranchId::Canopy,
            NetworkUpgrade::Nu5 => BranchId::Nu5,
            NetworkUpgrade::Nu6 => BranchId::Nu6,
            NetworkUpgrade::Nu6_1 => BranchId::Nu6_1,
            #[cfg(zcash_unstable = "nu7")]
            NetworkUpgrade::Nu7 => BranchId::Nu7,
            #[cfg(zcash_unstable = "zfuture")]
            NetworkUpgrade::ZFuture => BranchId::ZFuture,
        }
    }
}

/// The network upgrades on the Zcash chain in order of activation.
///
/// This order corresponds to the activation heights, but because Rust enums are
/// full-fledged algebraic data types, we need to define it manually.
const UPGRADES_IN_ORDER: &[NetworkUpgrade] = &[
    NetworkUpgrade::Overwinter,
    NetworkUpgrade::Sapling,
    NetworkUpgrade::Ycash,
    NetworkUpgrade::Blossom,
    NetworkUpgrade::Heartwood,
    NetworkUpgrade::Canopy,
    NetworkUpgrade::Nu5,
    NetworkUpgrade::Nu6,
    NetworkUpgrade::Nu6_1,
    #[cfg(zcash_unstable = "nu7")]
    NetworkUpgrade::Nu7,
];

/// The "grace period" defined in [ZIP 212].
///
/// [ZIP 212]: https://zips.z.cash/zip-0212#changes-to-the-process-of-receiving-sapling-or-orchard-notes
pub const ZIP212_GRACE_PERIOD: u32 = 32256;

/// A globally-unique identifier for a set of consensus rules within the Zcash chain.
///
/// Each branch ID in this enum corresponds to one of the epochs between a pair of Zcash
/// network upgrades. For example, `BranchId::Overwinter` corresponds to the blocks
/// starting at Overwinter activation, and ending the block before Sapling activation.
///
/// The main use of the branch ID is in signature generation: transactions commit to a
/// specific branch ID by including it as part of [`signature_hash`]. This ensures
/// two-way replay protection for transactions across network upgrades.
///
/// See [ZIP 200](https://zips.z.cash/zip-0200) for more details.
///
/// [`signature_hash`]: https://docs.rs/zcash_primitives/latest/zcash_primitives/transaction/sighash/fn.signature_hash.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchId {
    /// The consensus rules at the launch of Zcash.
    Sprout,
    /// The consensus rules deployed by [`NetworkUpgrade::Overwinter`].
    Overwinter,
    /// The consensus rules deployed by [`NetworkUpgrade::Sapling`].
    Sapling,
    /// The consensus rules deployed by [`NetworkUpgrade::Ycash`] — active only on Ycash.
    Ycash,
    /// The consensus rules deployed by [`NetworkUpgrade::Blossom`].
    Blossom,
    /// The Blossom-era consensus rules on the Ycash chain (distinct branch ID from Zcash's
    /// [`BranchId::Blossom`] for two-way replay protection across the fork).
    YBlossom,
    /// The consensus rules deployed by [`NetworkUpgrade::Heartwood`].
    Heartwood,
    /// The Heartwood-era consensus rules on the Ycash chain.
    YHeartwood,
    /// The consensus rules deployed by [`NetworkUpgrade::Canopy`].
    Canopy,
    /// The Canopy-era consensus rules on the Ycash chain.
    YCanopy,
    /// The consensus rules deployed by [`NetworkUpgrade::Nu5`].
    Nu5,
    /// The consensus rules deployed by [`NetworkUpgrade::Nu6`].
    Nu6,
    /// The consensus rules deployed by [`NetworkUpgrade::Nu6_1`].
    Nu6_1,
    /// The consensus rules to be deployed by [`NetworkUpgrade::Nu7`].
    #[cfg(zcash_unstable = "nu7")]
    Nu7,
    /// Candidates for future consensus rules; this branch will never
    /// activate on mainnet.
    #[cfg(zcash_unstable = "zfuture")]
    ZFuture,
}

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(BranchId);

impl TryFrom<u32> for BranchId {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BranchId::Sprout),
            0x5ba8_1b19 => Ok(BranchId::Overwinter),
            0x76b8_09bb => Ok(BranchId::Sapling),
            0x374d_694f => Ok(BranchId::Ycash),
            0x2bb4_0e60 => Ok(BranchId::Blossom),
            0x8e47_1bd6 => Ok(BranchId::YBlossom),
            0xf5b9_230b => Ok(BranchId::Heartwood),
            0x6631_4da3 => Ok(BranchId::YHeartwood),
            0xe9ff_75a6 => Ok(BranchId::Canopy),
            0x19bd_2d2f => Ok(BranchId::YCanopy),
            0xc2d6_d0b4 => Ok(BranchId::Nu5),
            0xc8e7_1055 => Ok(BranchId::Nu6),
            0x4dec_4df0 => Ok(BranchId::Nu6_1),
            #[cfg(zcash_unstable = "nu7")]
            0xffff_ffff => Ok(BranchId::Nu7),
            #[cfg(zcash_unstable = "zfuture")]
            0xffff_ffff => Ok(BranchId::ZFuture),
            _ => Err("Unknown consensus branch ID"),
        }
    }
}

impl From<BranchId> for u32 {
    fn from(consensus_branch_id: BranchId) -> u32 {
        match consensus_branch_id {
            BranchId::Sprout => 0,
            BranchId::Overwinter => 0x5ba8_1b19,
            BranchId::Sapling => 0x76b8_09bb,
            BranchId::Ycash => 0x374d_694f,
            BranchId::Blossom => 0x2bb4_0e60,
            BranchId::YBlossom => 0x8e47_1bd6,
            BranchId::Heartwood => 0xf5b9_230b,
            BranchId::YHeartwood => 0x6631_4da3,
            BranchId::Canopy => 0xe9ff_75a6,
            BranchId::YCanopy => 0x19bd_2d2f,
            BranchId::Nu5 => 0xc2d6_d0b4,
            BranchId::Nu6 => 0xc8e7_1055,
            BranchId::Nu6_1 => 0x4dec_4df0,
            #[cfg(zcash_unstable = "nu7")]
            BranchId::Nu7 => 0xffff_ffff,
            #[cfg(zcash_unstable = "zfuture")]
            BranchId::ZFuture => 0xffff_ffff,
        }
    }
}

impl BranchId {
    /// Returns the branch ID corresponding to the consensus rule set that is active at
    /// the given height.
    ///
    /// This is the branch ID that should be used when creating transactions.
    pub fn for_height<P: Parameters>(parameters: &P, height: BlockHeight) -> Self {
        for nu in UPGRADES_IN_ORDER.iter().rev() {
            if parameters.is_nu_active(*nu, height) {
                return parameters.branch_id(*nu);
            }
        }

        // Sprout rules apply before any network upgrade
        BranchId::Sprout
    }

    /// Returns the range of heights for the consensus epoch associated with this branch id.
    ///
    /// The resulting tuple implements the [`RangeBounds<BlockHeight>`] trait.
    pub fn height_range<P: Parameters>(&self, params: &P) -> Option<impl RangeBounds<BlockHeight>> {
        self.height_bounds(params).map(|(lower, upper)| {
            (
                Bound::Included(lower),
                upper.map_or(Bound::Unbounded, Bound::Excluded),
            )
        })
    }

    /// Returns the range of heights for the consensus epoch associated with this branch id.
    ///
    /// The return type of this value is slightly more precise than [`Self::height_range`]:
    /// - `Some((x, Some(y)))` means that the consensus rules corresponding to this branch id
    ///   are in effect for the range `x..y`
    /// - `Some((x, None))` means that the consensus rules corresponding to this branch id are
    ///   in effect for the range `x..`
    /// - `None` means that the consensus rules corresponding to this branch id are never in effect.
    pub fn height_bounds<P: Parameters>(
        &self,
        params: &P,
    ) -> Option<(BlockHeight, Option<BlockHeight>)> {
        if matches!(self, BranchId::Sprout) {
            return params
                .activation_height(NetworkUpgrade::Overwinter)
                .map(|upper| (BlockHeight(0), Some(upper)));
        }

        // Walk the upgrade list and find the upgrade that maps to this branch id on the
        // given network. The mapping is network-dependent (e.g. on Ycash,
        // `NetworkUpgrade::Blossom` maps to `BranchId::YBlossom` rather than
        // `BranchId::Blossom`), so we delegate via `parameters.branch_id`. The upper bound
        // is the activation height of the next upgrade that is actually activated on this
        // network, which lets post-Sapling bounds skip over Ycash on Zcash and skip over
        // Zcash's Blossom on Ycash.
        for (idx, nu) in UPGRADES_IN_ORDER.iter().enumerate() {
            if params.branch_id(*nu) != *self {
                continue;
            }
            let lower = params.activation_height(*nu)?;
            let upper = UPGRADES_IN_ORDER[idx + 1..]
                .iter()
                .find_map(|next| params.activation_height(*next));
            return Some((lower, upper));
        }
        None
    }

    pub fn sprout_uses_groth_proofs(&self) -> bool {
        !matches!(self, BranchId::Sprout | BranchId::Overwinter)
    }
}

#[cfg(any(test, feature = "test-dependencies"))]
pub mod testing {
    use proptest::sample::select;
    use proptest::strategy::{Just, Strategy};

    use super::{BlockHeight, BranchId, Parameters};

    pub fn arb_branch_id() -> impl Strategy<Value = BranchId> {
        select(vec![
            BranchId::Sprout,
            BranchId::Overwinter,
            BranchId::Sapling,
            BranchId::Ycash,
            BranchId::Blossom,
            BranchId::YBlossom,
            BranchId::Heartwood,
            BranchId::YHeartwood,
            BranchId::Canopy,
            BranchId::YCanopy,
            BranchId::Nu5,
            BranchId::Nu6,
            BranchId::Nu6_1,
            #[cfg(zcash_unstable = "nu7")]
            BranchId::Nu7,
            #[cfg(zcash_unstable = "zfuture")]
            BranchId::ZFuture,
        ])
    }

    pub fn arb_height<P: Parameters>(
        branch_id: BranchId,
        params: &P,
    ) -> impl Strategy<Value = Option<BlockHeight>> {
        branch_id
            .height_bounds(params)
            .map_or(Strategy::boxed(Just(None)), |(lower, upper)| {
                Strategy::boxed(
                    (lower.0..upper.map_or(u32::MAX, |u| u.0)).prop_map(|h| Some(BlockHeight(h))),
                )
            })
    }

    #[cfg(feature = "test-dependencies")]
    impl incrementalmerkletree_testing::TestCheckpoint for BlockHeight {
        fn from_u64(value: u64) -> Self {
            BlockHeight(u32::try_from(value).expect("Test checkpoint ids do not exceed 32 bits"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlockHeight, BranchId, NetworkUpgrade, Parameters, MAIN_NETWORK, UPGRADES_IN_ORDER,
        YCASH_MAIN_NETWORK,
    };

    /// UPGRADES_IN_ORDER is a global list; individual networks may skip some upgrades
    /// (e.g. Zcash networks skip `NetworkUpgrade::Ycash`, Ycash networks stop at Canopy).
    /// The invariant we check per network is: when two *consecutive list entries are both
    /// activated* on that network, they must be in ascending activation order.
    fn check_nu_ordering<P: Parameters>(params: &P) {
        let mut prev: Option<(NetworkUpgrade, BlockHeight)> = None;
        for nu in UPGRADES_IN_ORDER {
            if let Some(h) = params.activation_height(*nu) {
                if let Some((prev_nu, prev_h)) = prev {
                    assert!(
                        prev_h < h,
                        "{} ({}) should precede {} ({}) in UPGRADES_IN_ORDER",
                        prev_nu,
                        prev_h,
                        nu,
                        h,
                    );
                }
                prev = Some((*nu, h));
            }
        }
    }

    #[test]
    fn nu_ordering() {
        check_nu_ordering(&MAIN_NETWORK);
        check_nu_ordering(&YCASH_MAIN_NETWORK);
    }

    #[test]
    fn nu_is_active() {
        assert!(!MAIN_NETWORK.is_nu_active(NetworkUpgrade::Overwinter, BlockHeight(0)));
        assert!(!MAIN_NETWORK.is_nu_active(NetworkUpgrade::Overwinter, BlockHeight(347_499)));
        assert!(MAIN_NETWORK.is_nu_active(NetworkUpgrade::Overwinter, BlockHeight(347_500)));
    }

    #[test]
    fn branch_id_from_u32() {
        assert_eq!(BranchId::try_from(0), Ok(BranchId::Sprout));
        assert!(BranchId::try_from(1).is_err());
    }

    #[test]
    fn branch_id_for_height() {
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(0)),
            BranchId::Sprout,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(419_199)),
            BranchId::Overwinter,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(419_200)),
            BranchId::Sapling,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(903_000)),
            BranchId::Heartwood,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(1_046_400)),
            BranchId::Canopy,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(1_687_104)),
            BranchId::Nu5,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(2_726_399)),
            BranchId::Nu5,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(2_726_400)),
            BranchId::Nu6,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(3_146_400)),
            BranchId::Nu6_1,
        );
        assert_eq!(
            BranchId::for_height(&MAIN_NETWORK, BlockHeight(5_000_000)),
            BranchId::Nu6_1,
        );
    }

    #[test]
    fn ycash_branch_id_for_height() {
        // Pre-Sapling on Ycash mainnet.
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(347_500)),
            BranchId::Overwinter,
        );
        // Sapling epoch — active from 419_200 until the Ycash fork at 570_000.
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(419_200)),
            BranchId::Sapling,
        );
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(569_999)),
            BranchId::Sapling,
        );
        // Ycash epoch — activation height 570_000 up to Blossom at 1_100_000.
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(570_000)),
            BranchId::Ycash,
        );
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(1_099_999)),
            BranchId::Ycash,
        );
        // Post-fork Blossom / Heartwood / Canopy use Y-branch IDs (distinct from Zcash's
        // for two-way replay protection across the chain split).
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(1_100_000)),
            BranchId::YBlossom,
        );
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(1_100_003)),
            BranchId::YHeartwood,
        );
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(1_100_006)),
            BranchId::YCanopy,
        );
        // Well past Canopy; Ycash never activated NU5 so YCanopy stays in effect.
        assert_eq!(
            BranchId::for_height(&YCASH_MAIN_NETWORK, BlockHeight(5_000_000)),
            BranchId::YCanopy,
        );
    }

    #[test]
    fn ycash_branch_id_u32_roundtrip() {
        // The Y-variant consensus branch IDs must round-trip through their u32 wire
        // representations. Values from hhanh00/librustzcash@ywallet.
        for (bid, expected) in [
            (BranchId::Ycash, 0x374d_694fu32),
            (BranchId::YBlossom, 0x8e47_1bd6),
            (BranchId::YHeartwood, 0x6631_4da3),
            (BranchId::YCanopy, 0x19bd_2d2f),
        ] {
            let encoded: u32 = bid.into();
            assert_eq!(encoded, expected);
            assert_eq!(BranchId::try_from(encoded).unwrap(), bid);
        }
    }

    #[test]
    fn ycash_height_bounds() {
        // On Ycash mainnet, Sapling's upper bound must be the Ycash activation (570_000),
        // not Blossom's heights. This verifies the `height_bounds` rewrite correctly
        // skips over upgrades that are not activated on this network.
        let (lower, upper) = BranchId::Sapling
            .height_bounds(&YCASH_MAIN_NETWORK)
            .expect("Sapling is active on Ycash mainnet");
        assert_eq!(lower, BlockHeight(419_200));
        assert_eq!(upper, Some(BlockHeight(570_000)));

        // The plain `Blossom` branch is never the active branch on Ycash (the network
        // uses `YBlossom` instead). Its bounds should be None.
        assert!(BranchId::Blossom
            .height_bounds(&YCASH_MAIN_NETWORK)
            .is_none());

        // Conversely, `YBlossom` is not a branch on Zcash mainnet.
        assert!(BranchId::YBlossom.height_bounds(&MAIN_NETWORK).is_none());

        // `YCanopy` has an unbounded upper on Ycash because NU5 never activates.
        let (lower, upper) = BranchId::YCanopy
            .height_bounds(&YCASH_MAIN_NETWORK)
            .expect("YCanopy is active on Ycash mainnet");
        assert_eq!(lower, BlockHeight(1_100_006));
        assert_eq!(upper, None);
    }
}
