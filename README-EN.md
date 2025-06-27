# Astro Kits
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/astro-xao/astrokits#license)
[![Docs](https://docs.rs/astrokits/badge.svg)](https://docs.rs/astrokits/latest/astrokits/)
[![CI](https://github.com/astro-xao/astrokits/workflows/CI/badge.svg)](https://github.com/astro-xao/astrokits/actions)

`astro-kits` is a collection of Rust toolkits for astronomical calculations. It mainly includes the following components:

- [`calceph-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/calceph-sys): Rust bindings for the Calceph C library, providing high-precision ephemeris calculations.
- [`libcspice-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/libcspice-sys): Rust bindings for the NAIF SPICE C library, supporting geometric computations for space science missions.
- [`supernovas-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/supernovas-sys): Rust bindings for the SuperNovas C library, offering astronomical calculations related to stars and supernovae.

These toolkits provide Rust developers with efficient and reliable astronomical computation capabilities.

## Usage
Enable source compilation features if you do not have `cspice`, `calceph`, or `supernovas` installed locally.
```
[dependencies.astrokits]
version = "0.1.2"
features = [
    "calceph",          # Include calceph support
    "cspice",           # Include cspice support
    "novas",            # Include supernovas support
    "build-src",        # Build from source, may take longer the first time
]
```
If you have `cspice`, `calceph`, and `supernovas` installed locally and have set the installation paths in the `CSPICE_DIR`, `CALCEPH_DIR`, and `SUPERNOVAS_DIR` environment variables, use the following configuration:
```
[dependencies.astrokits]
version = "0.1.2"
features = [
    "calceph",      # Include calceph support
    "cspice",       # Include cspice support
    "novas",        # Include supernovas support
]
```