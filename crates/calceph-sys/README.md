# calceph-sys

如果提供 `CALCEPH_DIR` 环境变量指向本地安装的 `calceph` 目录，则不 `build.rs` 不会从源码构建，只会依赖 `inlude` 目录下的头文件构建 FFI bindings。
如果需要从源码构建, 则需要提供 `download-src` 特性，该特性默认没有启用。
```
[dependencies]
calceph-sys = { version = "0.1.0", features = ["download-src"] }
```