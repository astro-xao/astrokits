use libcspice_sys::{
    furnsh_c, spkgeo_c, spkpos_c, str2et_c, SpiceDouble,
};
use std::ffi::CString;

fn main() {
    let eph_de405 = std::env::var("EPH_DE405").unwrap();
    let eph_lps = std::env::var("EPH_LPS").unwrap();
    unsafe {
        let bsp_str = CString::new(eph_de405).unwrap();
        furnsh_c(bsp_str.as_ptr());
        // Load leap seconds file
        let leap_str = CString::new(eph_lps).unwrap();
        furnsh_c(leap_str.as_ptr());
    }

    // Define date and time
    let mut et: SpiceDouble = 0.0;
    let date_str = CString::new("2025-03-04 09:04:47").unwrap();
    unsafe {
        str2et_c(date_str.as_ptr(), &mut et);
    }

    // Define variables to store position and velocity
    let mut state0 = [0.0f64; 6];
    let mut state1 = [0.0f64; 6];
    let mut lt: SpiceDouble = 0.0;

    unsafe {
        spkgeo_c(399, et, CString::new("J2000").unwrap().as_ptr(), 10, state0.as_mut_ptr(), &mut lt);
        spkpos_c(
            CString::new("EARTH BARYCENTER").unwrap().as_ptr(),
            et,
            CString::new("J2000").unwrap().as_ptr(),
            CString::new("NONE").unwrap().as_ptr(),
            CString::new("SUN").unwrap().as_ptr(),
            state1.as_mut_ptr(),
            &mut lt,
        );
    }

    // Output Earth's position and velocity
    println!("[SPICE]位置/速度 (公里/秒):");
    println!("X: {:20.8} dX: {:10.8}", state0[0], state0[3]);
    println!("Y: {:20.8} dY: {:10.8}", state0[1], state0[4]);
    println!("Z: {:20.8} dZ: {:10.8}", state0[2], state0[5]);
}
