// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

//! Deploy other contracts.
//!
//! Currently this module only supports low-level contract creation via [`RawDeploy`],
//! but word is being done to introduce high-level deployment patterns.

pub use raw::RawDeploy;

mod raw;
