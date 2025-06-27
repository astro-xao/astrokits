# Astro Kits
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/astro-xao/astrokits#license)
[![Docs](https://docs.rs/astrokits/badge.svg)](https://docs.rs/astrokits/latest/astrokits/)
[![CI](https://github.com/astro-xao/astrokits/workflows/CI/badge.svg)](https://github.com/astro-xao/astrokits/actions)

`astro-kits` 是一个集合了多种用于天文计算的 Rust 平台套件。主要包含以下组件：

- [`calceph-sys`](https://github.com/astro-xao/astrokits/calceph-sys)：Calceph C 库的 Rust 绑定，用于高精度天体历算。
- [`libcspice-sys`](https://github.com/astro-xao/astrokits/libcspice-sys)：NAIF SPICE C 库的 Rust 绑定，支持空间科学任务的几何计算。
- [`supernovas-sys`](https://github.com/astro-xao/astrokits/supernovas-sys)：SuperNovas C 库的 Rust 绑定，提供恒星和超新星相关的天文计算。

这些套件为 Rust 开发者提供了高效、可靠的天文计算能力。