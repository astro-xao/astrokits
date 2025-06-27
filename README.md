# Astro Kits
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/astro-xao/astrokits#license)
[![Docs](https://docs.rs/astrokits/badge.svg)](https://docs.rs/astrokits/latest/astrokits/)
[![CI](https://github.com/astro-xao/astrokits/workflows/CI/badge.svg)](https://github.com/astro-xao/astrokits/actions)

[README EN](./README-EN.md)

`astro-kits` 是一个集合了多种用于天文计算的 Rust 平台套件。主要包含以下组件：

- [`calceph-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/calceph-sys)：Calceph C 库的 Rust 绑定，用于高精度天体历算。
- [`libcspice-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/libcspice-sys)：NAIF SPICE C 库的 Rust 绑定，支持空间科学任务的几何计算。
- [`supernovas-sys`](https://github.com/astro-xao/astrokits/tree/main/crates/supernovas-sys)：SuperNovas C 库的 Rust 绑定，提供恒星和超新星相关的天文计算。

这些套件为 Rust 开发者提供了高效、可靠的天文计算能力。

## 使用
本地没有安装 `cspice` `calceph` `supernovas` 时开启源码编译特性。
```
[dependencies.astrokits]
version = "0.1.2"
features = [
    "calceph",          # 包含 calceph 功能
    "cspice",           # 包含 cspice 功能
    "novas",            # 包含 supernovas 功能
    "build-src",        # 从源码编译，第一次耗时较长
]
```
如果本地已经安装 `cspice` `calceph` `supernovas`，并且已经设置安装位置到 `CSPICE_DIR` `CALCEPH_DIR` `SUPERNOVAS_DIR` 环境变量，可以使用如下配置:
```
[dependencies.astrokits]
version = "0.1.2"
features = [
    "calceph",      # 包含 calceph 功能
    "cspice",       # 包含 cspice 功能
    "novas",        # 包含 supernovas 功能
]
```