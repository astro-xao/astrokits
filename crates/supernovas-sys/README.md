# supernovas-sys
`supernovas-sys` 是一个 Rust FFI 接口库，绑定 [calceph](https://gitlab.obspm.fr/imcce_calceph/calceph)、[cspice](https://naif.jpl.nasa.gov/naif/toolkit_C.html) 和 [supernovas](https://github.com/Smithsonian/SuperNOVAS/) 等 C 语言天文计算库，为 Rust 提供高性能的天体力学和天文数据处理能力，适用于科研、工程和教育等需要精确天文计算的项目。

# 示例
```
cargo run --example calceph
```
```
cargo run --example cspice
```
```
cargo run --example high-z
```
```
cargo run --example rise-set
```
