/*
    Appellation: platform <module>
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
pub struct PlatformCmd {
    #[clap(subcommand)]
    pub args: Option<PlatformOpts>,
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
pub enum PlatformOpts {
    Connect {
        #[clap(long, short)]
        target: Option<String>,
    },
}

impl PlatformOpts {
    pub fn connect(target: Option<String>) -> Self {
        Self::Connect { target }
    }
}

impl Default for PlatformOpts {
    fn default() -> Self {
        Self::Connect { target: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_command() {
        let args = PlatformOpts::Connect {
            target: Some("10".to_string()),
        };
        assert_eq!(args.to_string(), "connect");
        println!("{}", args);
    }
}
