/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod build;
pub mod deploy;
pub mod platform;
pub mod system;

pub(crate) mod prelude {
    pub use super::build::{BuildCmd, BuildOpts};
    pub use super::deploy::{DeployCmd, DeployOpts};
    pub use super::platform::{PlatformCmd, PlatformOpts};
    pub use super::system::{SystemCmd, SystemOpts};
    pub use super::Options;
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumString,
    strum::EnumIter,
)]
#[repr(C)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Options {
    Build(BuildCmd),
    Deploy(DeployCmd),
    Platform(PlatformCmd),
    Sys(SystemCmd),
}
