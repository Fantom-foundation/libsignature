libsignature
============
[![Build Status](https://travis-ci.org/Fantom-foundation/libsignature.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libsignature)

libsignature in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Description

This crate defines a set of commonly used traits which can be used for various consensus
implementations. The crate defines two traits: ConsensusConfiguration and Consensus. The crate
also defines a base struct (BaseConsensusPeer) which can be used between multiple consensus algorithms.

For an example of an implementation of the traits, refer to the libsignature-dag repository:
https://github.com/Fantom-foundation/libsignature-dag.

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libsignature && cd libsignature
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```

### Example

#### Prelude
``` rust
use libsignature::{PublicKey, SecretKey, Signature;
```

**An example how to verify signature**
``` rust
        for (signatory, signature) in event.signatures.iter() {
            let peer = self.conf.read().unwrap().peers.find_peer(signatory)?;
            let res = signature.verify(event.get_hash(), peer.get_public_key())?;
            if !res {
                return Ok(false);
            }
        }
```

**An example how to sign and create `Signature`**
```rust
impl<SK, PK, Sig> Struct<SK, PK, Sig>
where Sig: Signature<Hash=EventHash, PublicKey=PK, SecretKey=SK> {
    fn some_function() {
        let signature = Sig::sign(self.get_hash(), self.get_secret_key());
    }
}
```