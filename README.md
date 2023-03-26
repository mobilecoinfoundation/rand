# mc-rand

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->![Architecture: any][arch-image]<!--
-->[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->[![CodeCov Status][codecov-image]][codecov-link]<!--
-->[![GitHub Workflow Status][gha-image]][gha-link]<!--
-->[![Contributor Covenant][conduct-image]][conduct-link]

A platform abstraction layer providing a cryptographic RNG, `McRng`.

## Example usage

Example usage:

```rust
use mc_rand::{McRng, RngCore}

pub fn my_func() -> (u64, u64) {
    let mut rng = McRng::default();
    let k0 = rng.next_u64();
    let k1 = rng.next_u64();
    (k0, k1)
}
```

## What it does

This project has evolved considerably as cargo has gotten more bug fixes and features.

Today, what it does is:

* On targets with CPU feature `rdrand`, `McRng` resolves to `RdRandRng`, which uses
  CPU intrinsics to call `RDRAND` directly. This implementation was audited by NCC group.
* When this feature is not present, but the target_arch is `wasm_32`, `McRng` resolves to `OsRng` from rand crate.
* When neither of these is the case, `McRng` resolves to `ThreadRng`.

On targets with `rdrand`, this crate does not pull in the standard library.
On targets without `rdrand`, the feature `rand/std` will be enabled.

## Motivation

`McRng` was created initially because MobileCoin builds SGX enclave software in
a strict `no_std` environment. Enclaves are generally supposed to get randomness
from the CPU via `RDRAND` and not from the OS, because the OS is untrusted in the
SGX security model.

This creates the following needs:

* An RNG needs to exist in enclave implementations which resolves to `RDRAND`.
* Ideally the same RNG also works in unit-test builds of the enclave-impl crate without further configuration if you are working on x86.
* Some libraries are meant to be consumed both by enclaves in servers, and in clients, which may not be x86. See e.g. "common" crate,
  which also provided a hashmap for use in the enclave and out of the enclave, and which requires randomness to avoid HASH-DOS.
* Four years ago, before the `resolver = 2` innovation, cargo would unify features across build-dependencies and target dependencies,
  so if anything in your `build.rs` pulled in `rand` then you would get the standard library, which made common libraries like `rand`
  toxic to our enclave builds.

We wanted to have an RNG type that any of these users can consume, that will be secure and do the right thing on each platform
without requiring explicit configuration or other toil from developers.

Because none of the existing RNG libraries quite provided this, we made `mc-rand`.

## Future directions

`McRng` fills a niche that isn't quite filled by `OsRng` or `ThreadRng` or other popular crates, and has been audited and battle-tested in production for years.

Feel free to use `mc-rand` knowing that it will usually do the right thing:

* When working on x86 on enclave implementations, it's great, because your cargo tests and benchmarks will use the same implementation that your code will use in production.
* On other targets, it will still use the most generally recommendable crypographic RNG for that platform.

As other targets arise that are of interest, we are happy to improve support for them.

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://discord.gg/mobilecoin
[license-image]: https://img.shields.io/crates/l/mc-rand?style=flat-square
[arch-image]: https://img.shields.io/badge/arch-any-brightgreen?style=flat-square
[crate-image]: https://img.shields.io/crates/v/mc-rand.svg?style=flat-square
[crate-link]: https://crates.io/crates/mc-rand
[docs-image]: https://img.shields.io/docsrs/mc-rand?style=flat-square
[docs-link]: https://docs.rs/crate/mc-rand
[deps-image]: https://deps.rs/repo/github/mobilecoinfoundation/rand/status.svg?style=flat-square
[deps-link]: https://deps.rs/repo/github/mobilecoinfoundation/rand
[codecov-image]: https://img.shields.io/codecov/c/github/mobilecoinfoundation/rand/main?style=flat-square
[codecov-link]: https://codecov.io/gh/mobilecoinfoundation/rand
[gha-image]: https://img.shields.io/github/actions/workflow/status/mobilecoinfoundation/rand/ci.yaml?branch=main&style=flat-square
[gha-link]: https://github.com/mobilecoinfoundation/rand/actions/workflows/ci.yaml?query=branch%3Amain
[conduct-link]: CODE_OF_CONDUCT.md
[conduct-image]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square
