/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize,
)]
pub struct DeployCmd {
    #[clap(subcommand)]
    pub args: Option<DeployOpts>,
    #[clap(long, short, default_value = ".artifacts")]
    pub artifacts: String,
    #[clap(long, short)]
    pub context: Option<String>,
    #[clap(long, short, default_value = ".")]
    pub workdir: String,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    Subcommand,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DeployOpts {
    #[default]
    Web {
        #[clap(long, short)]
        url: String,
    },
}
