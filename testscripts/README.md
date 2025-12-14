## Testscripts

Collection of cargo commands to execute a bunch of integration tests also used by the github actions steps.
The scripts assume that they are executed with this folder as current working directory!


### Other
The `other.sh` script requires:
- nightly toolchain
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [cargo-docs-rs](https://github.com/dtolnay/cargo-docs-rs)
- [cargo-msrv](https://github.com/foresterre/cargo-msrv)