/*
    Appellation: system <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Parser,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct SystemCmd {
    #[clap(subcommand)]
    pub args: Option<SystemOpts>,
    pub settings: String,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
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
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SystemOpts {
    Config {
        #[clap(long, short, default_value_t = String::from("Puzzled.toml"))]
        settings: String,
    },
}

impl SystemOpts {
    pub fn config(settings: String) -> Self {
        Self::Config { settings }
    }
}
