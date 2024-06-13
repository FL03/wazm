/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # wazm
//!
//! wazm aims to be an efficient virutalized environment orchestrating workloads across various surfaces.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros;

pub mod prelude {}
