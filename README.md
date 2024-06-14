# wazm

[![crates.io](https://img.shields.io/crates/v/wazm.svg)](https://crates.io/crates/wazm)
[![docs.rs](https://docs.rs/wazm/badge.svg)](https://docs.rs/wazm)

[![clippy](https://github.com/FL03/wazm/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/wazm/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/wazm/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/wazm/actions/workflows/rust.yml)

***

### _The library is currently in the early stages of development and is not yet ready for production use._

The `wazm` protocol provides efficient mechansisms for efficiently managing virualized workspaces. The protocol focuses on
being a light-weight

wazm aims to be an efficient virutalized environment orchestrating workloads across various surfaces.


## Features



## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/wazm.git
cd wazm
```

```bash
cargo build --features full -r --workspace
```

## Usage

### Example

```rust
    extern crate wazm;

    fn main() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        tracing::info!("Starting linear model example");


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
