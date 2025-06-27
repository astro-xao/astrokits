use std::{ffi::CString};
// use std::time::{SystemTime, UNIX_EPOCH};
use supernovas_sys as sn;
use sn::utils::{HMS, DMS};

const LEAP_SECONDS: i32 = 37; // [s] current leap seconds from IERS Bulletin C
const DUT1: f64 = 0.035044;      // [s] current UT1 - UTC time difference from IERS Bulletin A
const POLAR_DX: f64 = 142.0;  // [mas] Earth polar offset x
const POLAR_DY: f64 = 443.05;  // [mas] Earth polar offset y

fn main() {
    unsafe {

        // Enable debugging output
        sn::novas_debug(sn::novas_debug_mode_NOVAS_DEBUG_ON);

        // Load CSPICE kernel (ephemeris file)
        let kernel_path = CString::new(std::env::var("EPH_DE440S").unwrap()).unwrap();
        if sn::cspice_add_kernel(kernel_path.as_ptr()) != 0 {
            eprintln!("ERROR! could not open ephemeris data");
            std::process::exit(1);
        }

        // Use CSPICE as ephemeris provider
        sn::novas_use_cspice();

        // Set accuracy
        let accuracy = sn::novas_accuracy_NOVAS_FULL_ACCURACY;

        // Define Mars as the observed source
        let mut source = std::mem::zeroed::<sn::object>();
        if sn::make_planet(sn::novas_planet_NOVAS_SUN, &mut source) != 0 {
            eprintln!("ERROR! defining planet.");
            std::process::exit(1);
        }

        // Define observer on Earth's surface
        let mut obs = std::mem::zeroed::<sn::observer>();
        if sn::make_observer_on_surface(43.82441, 87.61390, 0.0, 0.0, 0.0, &mut obs) != 0 {
            eprintln!("ERROR! defining Earth-based observer location.");
            std::process::exit(1);
        }
        // if sn::make_observer_on_surface(50.7374, 7.0982, 60.0, 0.0, 0.0, &mut obs) != 0 {
        //     eprintln!("ERROR! defining Earth-based observer location.");
        //     std::process::exit(1);
        // }

        // Get current system time (UNIX time)
        // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // let unix_sec = now.as_secs() as i64;
        // let unix_nsec = now.subsec_nanos() as i32;

        // Set astrometric time of observation
        let mut obs_time = std::mem::zeroed::<sn::novas_timespec>();
        // Parse ISO date string to Julian Date using novas_parse_iso_date
        let iso_str = CString::new("2025-06-24T12:29:36Z").unwrap();
        let jd = sn::novas_parse_iso_date(iso_str.as_ptr(), std::ptr::null_mut());

        sn::novas_set_time(sn::novas_timescale_NOVAS_UTC, jd, LEAP_SECONDS, DUT1, &mut obs_time);

        // Initialize observing frame
        let mut obs_frame = std::mem::zeroed::<sn::novas_frame>();
        if sn::novas_make_frame(
            accuracy,
            &obs,
            &obs_time,
            POLAR_DX,
            POLAR_DY,
            &mut obs_frame,
        ) != 0 {
            eprintln!("ERROR! failed to define observing frame.");
            std::process::exit(1);
        }
        // calculate current position (ICRS)
        let mut current = std::mem::zeroed::<sn::sky_pos>();
        if sn::novas_sky_pos(
            &source,
            &obs_frame,
            sn::novas_reference_system_NOVAS_ICRS,
            &mut current,
        ) != 0
        {
            eprintln!("ERROR! failed to calculate current position.");
            std::process::exit(1);
        }

        println!(
            " RA = {:} h, Dec = {:} deg, rad_vel = {:.6} km/s",
            HMS::from(current.ra), DMS::from(current.dec), current.rv
        );

        // Calculate apparent position (CIRS)
        let mut apparent = std::mem::zeroed::<sn::sky_pos>();
        if sn::novas_sky_pos(
            &source,
            &obs_frame,
            sn::novas_reference_system_NOVAS_CIRS,
            &mut apparent,
        ) != 0 {
            eprintln!("ERROR! failed to calculate apparent position.");
            std::process::exit(1);
        }

        println!(
            " RA = {:} h, Dec = {:} deg, rad_vel = {:.6} km/s",
            HMS::from(apparent.ra), DMS::from(apparent.dec), apparent.rv
        );

        // Convert to horizontal coordinates (azimuth/elevation)
        let mut az = 0.0f64;
        let mut el = 0.0f64;
        if sn::novas_app_to_hor(
            &obs_frame,
            sn::novas_reference_system_NOVAS_CIRS,
            apparent.ra,
            apparent.dec,
            Some(sn::novas_standard_refraction),
            &mut az,
            &mut el,
        ) != 0 {
            eprintln!("ERROR! failed to calculate azimuth / elevation.");
            std::process::exit(1);
        }
        println!(" Az = {:} deg, El = {:} deg", DMS::from(az), DMS::from(el));
    }
}