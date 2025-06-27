#[cfg(feature = "cspice")]
pub mod cspice {
    pub use libcspice_sys::*;
}

#[cfg(feature = "calceph")]
pub mod calceph {
    pub use calceph_sys::*;
}

#[cfg(feature = "novas")]
pub mod supernvas {
    pub use supernovas_sys::*;
}