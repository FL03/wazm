/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::args::prelude::*;

pub mod args;

pub(crate) fn parse() -> CLI {
    CLI::parse()
}

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
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Option<Options>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl CLI {
    pub fn new() -> Self {
        Self {
            command: None,
            verbose: false,
        }
    }

    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }

    pub fn cmd(&self) -> Option<&Options> {
        self.command.as_ref()
    }

    pub fn cmd_exists(&self) -> bool {
        self.command.is_some()
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }
}

wazm::display!(json: CLI, BuildCmd, DeployCmd, PlatformCmd, SystemCmd);
