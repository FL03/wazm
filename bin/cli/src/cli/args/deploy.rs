/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Parser,
    serde::Deserialize,
    serde::Serialize,
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
pub enum DeployOpts {
    Web {
        #[clap(long, short)]
        url: String,
    },
}

/*
 ************* Implementations *************
*/

impl Default for DeployCmd {
    fn default() -> Self {
        Self {
            args: None,
            artifacts: ".artifacts".to_string(),
            context: None,
            workdir: ".".to_string(),
        }
    }
}
impl Default for DeployOpts {
    fn default() -> Self {
        Self::Web {
            url: "https://localhost:8080".to_string(),
        }
    }
}
