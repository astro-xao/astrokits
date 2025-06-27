# Astro Kits
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/astro-xao/astrokits#license)
[![Docs](https://docs.rs/astrokits/badge.svg)](https://docs.rs/astrokits/latest/astrokits/)
[![CI](https://github.com/astro-xao/astrokits/workflows/CI/badge.svg)](https://github.com/astro-xao/astrokits/actions)

[README 中文](./README-zh.md)

`astro-kits` is a collection of Rust toolkits for astronomical calculations. It mainly includes the following components:

- [`calceph-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/calceph-sys): Rust bindings for the Calceph C library, providing high-precision ephemeris calculations.
- [`libcspice-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/libcspice-sys): Rust bindings for the NAIF SPICE C library, supporting geometric computations for space science missions.
- [`supernovas-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/supernovas-sys): Rust bindings for the SuperNovas C library, offering astronomical calculations related to stars and supernovae.

These toolkits provide Rust developers with efficient and reliable astronomical computation capabilities.