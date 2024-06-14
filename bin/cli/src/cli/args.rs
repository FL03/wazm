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

impl Options {
    pub fn from_build(cmd: BuildCmd) -> Self {
        Self::Build(cmd)
    }

    pub fn from_deploy(cmd: DeployCmd) -> Self {
        Self::Deploy(cmd)
    }

    pub fn from_platform(cmd: PlatformCmd) -> Self {
        Self::Platform(cmd)
    }

    pub fn from_sys(cmd: SystemCmd) -> Self {
        Self::Sys(cmd)
    }
}

/*
 ************* Implementations *************
*/

macro_rules! impl_from {
    ($($variant:ident($cmd:ty)),* $(,)?) =>  {
        $(impl_from!(@impl $variant($cmd));)*
    };
    (@impl $variant:ident($cmd:ty)) => {
        impl From<$cmd> for Options {
            fn from(cmd: $cmd) -> Self {
                Self::$variant(cmd)
            }
        }
    };
}

impl_from! {
    Build(BuildCmd),
    Deploy(DeployCmd),
    Platform(PlatformCmd),
    Sys(SystemCmd),
}
