# ethp2p

<!-- [![Crates.io][crates-badge]][crates-url] -->
[![documentation](https://img.shields.io/badge/documentation-rustdoc-passing)](https://www.dancline.net/ethp2p)
[![MIT License](https://img.shields.io/github/license/rjected/ethp2p)](https://github.com/rjected/ethp2p/blob/main/LICENSE)
[![CI](https://github.com/rjected/ethp2p/actions/workflows/ci.yml/badge.svg)](https://github.com/rjected/ethp2p/actions/workflows/ci.yml)

<!-- [crates-badge]: https://img.shields.io/crates/v/ethp2p.svg -->
<!-- [crates-url]: https://crates.io/crates/ethp2p -->

P2P types and utilities for working with [`eth`](https://github.com/ethereum/devp2p) protocol
messages.

`ethp2p` is built on [`anvil`](https://github.com/foundry-rs/foundry/tree/master/anvil) types, and
implements [`fastrlp`](https://github.com/vorot93/fastrlp) traits for RLP encoding and decoding.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ethp2p = { git = "https://github.com/rjected/ethp2p" }
```

*Compiler support: requires rustc 1.62+*
