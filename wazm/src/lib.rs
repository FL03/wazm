/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # wazm
//!
//! wazm aims to be an efficient virutalized environment orchestrating workloads across various surfaces.
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "wazm"]

#[doc(inline)]
pub use wazm_core::*;

#[allow(unused_imports)]
pub mod prelude {
    pub use wazm_core::prelude::*;
}
