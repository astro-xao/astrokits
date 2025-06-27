#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod utils {
    pub struct HMS(pub i32, pub i32, pub f64);

    // impl from(h: f64) for HMS
    impl From<f64> for HMS {
        fn from(hour: f64) -> Self {
            let h = hour.floor() as i32;
            let m = ((hour - h as f64) * 60.0).floor() as i32;
            let s = (hour - h as f64 - m as f64 / 60.0) * 3600.0;
            HMS(h, m, s)
        }
    }

    impl std::fmt::Display for HMS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let HMS(h, m, s) = self;
            write!(f, "{:02}h {:02}m {:02.2}s", h, m, s)
        }
    }

    pub struct DMS(pub i32, pub i32, pub f64);

    // impl from(d: f64) for DMS
    impl From<f64> for DMS {
        fn from(deg: f64) -> Self {
            let d = deg.floor() as i32;
            let m = ((deg - d as f64) * 60.0).floor() as i32;
            let s = (deg - d as f64 - m as f64 / 60.0) * 3600.0;
            DMS(d, m, s)
        }
    }

    impl std::fmt::Display for DMS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let DMS(d, m, s) = self;
            write!(f, "{:02}° {:02}′ {:02.2}″", d, m, s)
        }
    }
}