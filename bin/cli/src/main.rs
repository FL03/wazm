/*
    Appellation: pzzld-cli <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/

pub mod cli;

fn main() -> anyhow::Result<()> {
    tracer();
    let cli = cli::parse();

    println!("{}", &cli);
    Ok(())
}

pub(crate) fn tracer() {
    use tracing::Level;
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::TRACE)
        .with_target(true)
        .init();
}
