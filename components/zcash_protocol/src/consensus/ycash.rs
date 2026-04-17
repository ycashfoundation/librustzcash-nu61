//! Ycash network parameters.
//!
//! Ycash is a chain-fork of Zcash that split at Zcash mainnet block 570_000 (June 2018).
//! It inherited Zcash's Overwinter/Sapling activations, then diverged: post-fork Blossom,
//! Heartwood, and Canopy activated at different heights under distinct consensus branch
//! IDs (`YBlossom`/`YHeartwood`/`YCanopy`) for two-way replay protection. Ycash has never
//! activated NU5 or later, so Orchard, Unified Addresses, and v5 transactions are not in
//! effect on the network.
//!
//! Activation heights are sourced from ycashd's `chainparams.cpp` (the C++ full node is
//! authoritative for Ycash consensus).

use super::{BlockHeight, BranchId, NetworkType, NetworkUpgrade, Parameters};

#[cfg(feature = "std")]
use memuse::DynamicUsage;

/// Marker struct for the Ycash production network.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct MainNetwork;

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(MainNetwork);

impl Parameters for MainNetwork {
    fn network_type(&self) -> NetworkType {
        NetworkType::YcashMain
    }

    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match nu {
            NetworkUpgrade::Overwinter => Some(BlockHeight(347_500)),
            NetworkUpgrade::Sapling => Some(BlockHeight(419_200)),
            NetworkUpgrade::Ycash => Some(BlockHeight(570_000)),
            NetworkUpgrade::Blossom => Some(BlockHeight(1_100_000)),
            NetworkUpgrade::Heartwood => Some(BlockHeight(1_100_003)),
            NetworkUpgrade::Canopy => Some(BlockHeight(1_100_006)),
            NetworkUpgrade::Nu5 => None,
            NetworkUpgrade::Nu6 => None,
            NetworkUpgrade::Nu6_1 => None,
            #[cfg(zcash_unstable = "nu7")]
            NetworkUpgrade::Nu7 => None,
            #[cfg(zcash_unstable = "zfuture")]
            NetworkUpgrade::ZFuture => None,
        }
    }

    fn branch_id(&self, nu: NetworkUpgrade) -> BranchId {
        match nu {
            NetworkUpgrade::Blossom => BranchId::YBlossom,
            NetworkUpgrade::Heartwood => BranchId::YHeartwood,
            NetworkUpgrade::Canopy => BranchId::YCanopy,
            _ => nu.branch_id(),
        }
    }
}

/// Marker struct for the Ycash test network.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct TestNetwork;

#[cfg(feature = "std")]
memuse::impl_no_dynamic_usage!(TestNetwork);

impl Parameters for TestNetwork {
    fn network_type(&self) -> NetworkType {
        NetworkType::YcashTest
    }

    fn activation_height(&self, nu: NetworkUpgrade) -> Option<BlockHeight> {
        match nu {
            NetworkUpgrade::Overwinter => Some(BlockHeight(207_500)),
            NetworkUpgrade::Sapling => Some(BlockHeight(280_000)),
            NetworkUpgrade::Ycash => Some(BlockHeight(510_248)),
            NetworkUpgrade::Blossom => Some(BlockHeight(661_610)),
            NetworkUpgrade::Heartwood => Some(BlockHeight(661_622)),
            NetworkUpgrade::Canopy => Some(BlockHeight(661_634)),
            NetworkUpgrade::Nu5 => None,
            NetworkUpgrade::Nu6 => None,
            NetworkUpgrade::Nu6_1 => None,
            #[cfg(zcash_unstable = "nu7")]
            NetworkUpgrade::Nu7 => None,
            #[cfg(zcash_unstable = "zfuture")]
            NetworkUpgrade::ZFuture => None,
        }
    }

    fn branch_id(&self, nu: NetworkUpgrade) -> BranchId {
        match nu {
            NetworkUpgrade::Blossom => BranchId::YBlossom,
            NetworkUpgrade::Heartwood => BranchId::YHeartwood,
            NetworkUpgrade::Canopy => BranchId::YCanopy,
            _ => nu.branch_id(),
        }
    }
}
