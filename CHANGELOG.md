# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [2.0.0] - 2026-10-08

### Changed

- Updated `rand` and `rand_core` to 0.10 and raised the minimum supported Rust
  version to 1.85.
- Replaced the `mc_rand::RngCore` re-export with `mc_rand::Rng`.
- Updated the WebAssembly entropy source from `OsRng` to an infallible adapter
  over `SysRng`.
