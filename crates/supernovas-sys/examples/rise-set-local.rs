use std::env;
use std::ffi::CString;
// use std::time::{SystemTime, UNIX_EPOCH};
use supernovas_sys::utils::DMS;
use supernovas_sys as sn;

const LEAP_SECONDS: i32 = 37; // [s] current leap seconds from IERS Bulletin C
const DUT1: f64 = 0.035044;      // [s] current UT1 - UTC time difference from IERS Bulletin A
const POLAR_DX: f64 = 142.0;  // [mas] Earth polar offset x
const POLAR_DY: f64 = 443.05;  // [mas] Earth polar offset y
// const LATITUDE: f64    =  43.863333_f64;  // Urumqi
// const LONGITUDE: f64   = 87.569722_f64; // Urumqi

fn main() {
    // Parse elevation argument
    let args: Vec<String> = env::args().collect();
    let el: f64 = if args.len() > 1 {
        args[1].parse().unwrap_or(0.0)
    } else {
        0.0
    };

    unsafe {
        let lat = sn::novas_str_degrees(CString::new("43 51 48.0").unwrap().as_ptr());
        let lon = sn::novas_str_degrees(CString::new("87 34 11.0").unwrap().as_ptr());
        println!(
            "Observer location: lon = {}, lat = {}",
            lat,
            lon
        );

        // Enable debugging
        sn::novas_debug(sn::novas_debug_mode_NOVAS_DEBUG_ON);

        // Load CSPICE kernel (ephemeris file)
        let kernel_path = CString::new(std::env::var("EPH_DE440").unwrap()).unwrap();
        if sn::cspice_add_kernel(kernel_path.as_ptr()) != 0 {
            eprintln!("ERROR! could not open ephemeris data");
            std::process::exit(1);
        }

        // Use CSPICE as ephemeris provider
        sn::novas_use_cspice();

        // Define Mars as the observed source
        let mut source = std::mem::zeroed::<sn::object>();
        if sn::make_planet(sn::novas_planet_NOVAS_SUN, &mut source) != 0 {
            eprintln!("ERROR! defining planet.");
            std::process::exit(1);
        }

        // Observer location (Bonn, Germany)
        let mut obs: sn::observer = std::mem::zeroed();
        if sn::make_observer_on_surface(lat, lon, 3.0, 20.0, 1013.0, &mut obs) != 0 {
            eprintln!("ERROR! defining Earth-based observer location.");
            std::process::exit(1);
        }

        // Get current system time
        // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // let unix_sec = now.as_secs() as i64;
        // let unix_nsec = now.subsec_nanos() as i32;
        let iso_str = CString::new("2025-06-25T12:00:00Z").unwrap();
        let jd = sn::novas_parse_iso_date(iso_str.as_ptr(), std::ptr::null_mut());

        // Set time of observation
        let mut obs_time: sn::novas_timespec = std::mem::zeroed();
        if sn::novas_set_time(sn::novas_timescale_NOVAS_UTC, jd, LEAP_SECONDS, DUT1, &mut obs_time) != 0 {
            eprintln!("ERROR! failed to set time of observation.");
            std::process::exit(1);
        }
        // if sn::novas_set_unix_time(unix_sec, unix_nsec, LEAP_SECONDS, DUT1, &mut obs_time) != 0 {
        //     eprintln!("ERROR! failed to set time of observation.");
        //     std::process::exit(1);
        // }

        // Use reduced accuracy (no planet provider)
        let accuracy = sn::novas_accuracy_NOVAS_FULL_ACCURACY;

        // Make observing frame
        let mut obs_frame: sn::novas_frame = std::mem::zeroed();
        if sn::novas_make_frame(
            accuracy,
            &obs,
            &obs_time,
            POLAR_DX,
            POLAR_DY,
            &mut obs_frame,
        ) != 0
        {
            eprintln!("ERROR! failed to define observing frame.");
            std::process::exit(1);
        }

        // Print source name and observer location
        let source_name = std::ffi::CStr::from_ptr(source.name.as_ptr());
        println!(
            "'{}' observed from lon = {}, lat = {}:",
            source_name.to_string_lossy(),
            DMS::from(obs.on_surf.longitude),
            DMS::from(obs.on_surf.latitude)
        );

        // Calculate next rise time
        let jd_utc = sn::novas_rises_above(
            el,
            &source,
            &obs_frame,
            Some(sn::novas_standard_refraction),
        );
        let mut ts: sn::novas_timespec = std::mem::zeroed();
        let mut timestamp = [0i8; 40];

        if jd_utc.is_nan() {
            println!(" will not rise above {:5.1} degrees", el);
        } else {
            sn::novas_set_time(
                sn::novas_timescale_NOVAS_UTC,
                jd_utc,
                LEAP_SECONDS,
                DUT1,
                &mut ts,
            );
            sn::novas_iso_timestamp(&ts, timestamp.as_mut_ptr(), timestamp.len().try_into().unwrap());
            let ts_str = std::ffi::CStr::from_ptr(timestamp.as_ptr()).to_string_lossy();
            println!(" will rise above {:5.1} degrees at  : {}", el, ts_str);
        }

        // Calculate next transit time
        let jd_utc = sn::novas_transit_time(&source, &obs_frame);
        sn::novas_set_time(
            sn::novas_timescale_NOVAS_UTC,
            jd_utc,
            LEAP_SECONDS,
            DUT1,
            &mut ts,
        );
        sn::novas_iso_timestamp(&ts, timestamp.as_mut_ptr(), timestamp.len().try_into().unwrap());
        let ts_str = std::ffi::CStr::from_ptr(timestamp.as_ptr()).to_string_lossy();
        println!(" will transit at                   : {}", ts_str);

        // Calculate next set time
        let jd_utc = sn::novas_sets_below(
            el,
            &source,
            &obs_frame,
            Some(sn::novas_standard_refraction),
        );
        if jd_utc.is_nan() {
            println!(" will not set below {:5.1} degrees", el);
        } else {
            sn::novas_set_time(
                sn::novas_timescale_NOVAS_UTC,
                jd_utc,
                LEAP_SECONDS,
                DUT1,
                &mut ts,
            );
            sn::novas_iso_timestamp(&ts, timestamp.as_mut_ptr(), timestamp.len().try_into().unwrap());
            let ts_str = std::ffi::CStr::from_ptr(timestamp.as_ptr()).to_string_lossy();
            println!(" will set below {:5.1} degrees at   : {}", el, ts_str);
        }
    }
}
