/*
    Appellation: build <module>
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
pub struct BuildCmd {
    #[clap(subcommand)]
    pub args: Option<BuildOpts>,
    #[clap(long, short, default_value = ".artifacts")]
    pub artifacts: String,
    #[clap(long, short)]
    pub context: Option<String>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[clap(long, short, default_value = ".")]
    pub workdir: String,
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
pub enum BuildOpts {
    Binary {
        #[clap(long, short)]
        target: Option<String>,
    },
}
