use supernovas_sys as sn;
use std::ffi::CString;
use std::os::raw::c_char;
// use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Constants for Earth orientation values
    const LEAP_SECONDS: i32 = 37;      // [s]
    const DUT1: f64 = 0.114;           // [s]
    const POLAR_DX: f64 = 230.0;       // [mas]
    const POLAR_DY: f64 = -62.0;       // [mas]    // SuperNOVAS variables
    let mut source = sn::object::default();
    let mut obs = sn::observer::default();
    let mut obs_time = sn::novas_timespec::default();
    let mut obs_frame = sn::novas_frame::default();
    let mut apparent = sn::sky_pos::default();
    let mut az: f64 = 0.0;
    let mut el: f64 = 0.0;

    // Enable debugging
    unsafe { sn::novas_debug(sn::novas_debug_mode_NOVAS_DEBUG_ON) };

    // Open ephemeris file with CALCEPH
    let ephem_path = CString::new(std::env::var("EPH_DE440S").unwrap()).unwrap();
    let de440 = unsafe { sn::calceph_open(ephem_path.as_ptr() as *const c_char) };
    if de440.is_null() {
        eprintln!("ERROR! could not open ephemeris data");
        std::process::exit(1);
    }

    // Use CALCEPH for major planets
    unsafe { sn::novas_use_calceph_planets(de440) };

    // Set accuracy
    let accuracy = sn::novas_accuracy_NOVAS_FULL_ACCURACY;

    // Define a major planet (Mars)
    let res = unsafe { sn::make_planet(sn::novas_planet_NOVAS_SUN, &mut source) };
    if res != 0 {
        eprintln!("ERROR! defining planet.");
        std::process::exit(1);
    }

    // Define observer on Earth's surface
    let res = unsafe {
        sn::make_observer_on_surface(
            50.7374, 7.0982, 60.0, 0.0, 0.0, &mut obs
        )
    };
    if res != 0 {
        eprintln!("ERROR! defining Earth-based observer location.");
        std::process::exit(1);
    }

    // Get current system time
    // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // let unix_sec = now.as_secs() as i64;
    // let unix_nsec = now.subsec_nanos() as i32;
    let unix_sec = 1750680744_i64;
    let unix_nsec = 238528900_i32;

    // Set time of observation
    let res = unsafe {
        sn::novas_set_unix_time(
            unix_sec,
            unix_nsec,
            LEAP_SECONDS,
            DUT1,
            &mut obs_time,
        )
    };
    if res != 0 {
        eprintln!("ERROR! failed to set time of observation.");
        std::process::exit(1);
    }

    // Initialize observing frame
    let res = unsafe {
        sn::novas_make_frame(
            accuracy,
            &obs,
            &obs_time,
            POLAR_DX,
            POLAR_DY,
            &mut obs_frame,
        )
    };
    if res != 0 {
        eprintln!("ERROR! failed to define observing frame.");
        std::process::exit(1);
    }

    // Calculate apparent position (CIRS)
    let res = unsafe {
        sn::novas_sky_pos(
            &source,
            &obs_frame,
            sn::novas_reference_system_NOVAS_CIRS,
            &mut apparent,
        )
    };
    if res != 0 {
        eprintln!("ERROR! failed to calculate apparent position.");
        std::process::exit(1);
    }

    println!(
        " RA = {:.9} h, Dec = {:.9} deg, rad_vel = {:.6} km/s",
        apparent.ra, apparent.dec, apparent.rv
    );

    // Convert to horizontal coordinates
    let res = unsafe {
        sn::novas_app_to_hor(
            &obs_frame,
            sn::novas_reference_system_NOVAS_CIRS,
            apparent.ra,
            apparent.dec,
            Some(sn::novas_standard_refraction),
            &mut az,
            &mut el,
        )
    };
    if res != 0 {
        eprintln!("ERROR! failed to calculate azimuth / elevation.");
        std::process::exit(1);
    }

    println!(" Az = {:.6} deg, El = {:.6} deg", az, el);
}
