use calceph_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};

fn printcoord(pv: [f64; 6], name: &str) {
    println!("{} :", name);
    for val in pv.iter() {
        println!("\t{:23.16E}", val);
    }
    println!();
}

fn main() {
    unsafe {
        let filename = CString::new(std::env::var("EXMAPLE1_DAT").unwrap()).unwrap();
        let res = calceph_sopen(filename.as_ptr());
        if res != 0 {
            println!("The ephemeris is already opened");

            let timescale = calceph_sgettimescale();
            if timescale != 0 {
                println!(
                    "timescale : {}",
                    if timescale == 1 { "TDB" } else { "TCB" }
                );
            }

            let mut jdfirst: c_double = 0.0;
            let mut jdlast: c_double = 0.0;
            let mut cont: c_int = 0;
            if calceph_sgettimespan(&mut jdfirst, &mut jdlast, &mut cont) != 0 {
                println!(
                    "data available between [ {}, {} ]. continuous={}",
                    jdfirst, jdlast, cont
                );
            }

            let mut au: c_double = 0.0;
            let mut emrat: c_double = 0.0;
            let mut gm_mer: c_double = 0.0;
            if calceph_sgetconstant(CString::new("AU").unwrap().as_ptr(), &mut au) != 0 {
                println!("AU={:23.16E}", au);
            }
            if calceph_sgetconstant(CString::new("EMRAT").unwrap().as_ptr(), &mut emrat) != 0 {
                println!("EMRAT={:23.16E}", emrat);
            }
            if calceph_sgetconstant(CString::new("GM_Mer").unwrap().as_ptr(), &mut gm_mer) != 0 {
                println!("GM_Mer={:23.16E}", gm_mer);
            }

            let jd0: c_double = 2442457.0;
            let dt: c_double = 0.5;
            let mut pv = [0f64; 6];

            // geocentric coordinates of the Moon
            calceph_scompute(jd0, dt, 10, 3, pv.as_mut_ptr());
            printcoord(pv, "geocentric coordinates of the Moon");

            // TT-TDB
            if calceph_scompute(jd0, dt, 16, 0, pv.as_mut_ptr()) != 0 {
                println!("TT-TDB = {:23.16E}", pv[0]);
            }

            println!("mars");
            // heliocentric coordinates of Mars
            calceph_scompute(jd0, dt, 4, 11, pv.as_mut_ptr());
            printcoord(pv, "heliocentric coordinates of Mars");

            // list of constants
            println!("list of constants");
            let count = calceph_sgetconstantcount();
            let mut nameconstant = [0 as c_char; CALCEPH_MAX_CONSTANTNAME as usize];
            let mut valueconstant: c_double = 0.0;
            for j in 1..=count {
                calceph_sgetconstantindex(
                    j,
                    nameconstant.as_mut_ptr(),
                    &mut valueconstant,
                );
                let cname = CStr::from_ptr(nameconstant.as_ptr()).to_string_lossy();
                println!("'{}'\t= {:23.16E}", cname, valueconstant);
            }

            calceph_sclose();
            println!("The ephemeris is already closed");
        } else {
            println!("The ephemeris can't be opened");
        }
    }
}
