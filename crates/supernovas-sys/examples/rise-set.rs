use std::env;
use std::ffi::CString;
use std::time::{SystemTime, UNIX_EPOCH};
use supernovas_sys as sn;

const LEAP_SECONDS: i32 = 37; // [s] current leap seconds from IERS Bulletin C
const DUT1: f64 = 0.114;      // [s] current UT1 - UTC time difference from IERS Bulletin A
const POLAR_DX: f64 = 230.0;  // [mas] Earth polar offset x
const POLAR_DY: f64 = -62.0;  // [mas] Earth polar offset y

fn main() {
    // Parse elevation argument
    let args: Vec<String> = env::args().collect();
    let el: f64 = if args.len() > 1 {
        args[1].parse().unwrap_or(0.0)
    } else {
        0.0
    };

    unsafe {
        // Enable debugging
        sn::novas_debug(sn::novas_debug_mode_NOVAS_DEBUG_ON);

        // Define sidereal source (Antares, B1950)
        let name = CString::new("Antares").unwrap();
        let catalog = CString::new("FK4").unwrap();
        let mut star: sn::cat_entry = std::mem::zeroed();
        let make_cat_entry_res = sn::make_cat_entry(
            name.as_ptr(),
            catalog.as_ptr(),
            1,
            16.43894213,
            -26.323094,
            -12.11,
            -23.30,
            5.89,
            -3.4,
            &mut star,
        );
        if make_cat_entry_res != 0 {
            eprintln!("ERROR! defining cat_entry.");
            std::process::exit(1);
        }

        // Convert to ICRS and wrap in object
        let sys = CString::new("B1950").unwrap();
        let mut source: sn::object = std::mem::zeroed();
        if sn::make_cat_object_sys(&star, sys.as_ptr(), &mut source) != 0 {
            eprintln!("ERROR! configuring observed object");
            std::process::exit(1);
        }

        // Observer location (Bonn, Germany)
        let mut obs: sn::observer = std::mem::zeroed();
        if sn::make_observer_on_surface(50.7374, 7.0982, 60.0, 0.0, 0.0, &mut obs) != 0 {
            eprintln!("ERROR! defining Earth-based observer location.");
            std::process::exit(1);
        }

        // Get current system time
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let unix_sec = now.as_secs() as i64;
        let unix_nsec = now.subsec_nanos() as i32;

        // Set time of observation
        let mut obs_time: sn::novas_timespec = std::mem::zeroed();
        if sn::novas_set_unix_time(unix_sec, unix_nsec, LEAP_SECONDS, DUT1, &mut obs_time) != 0 {
            eprintln!("ERROR! failed to set time of observation.");
            std::process::exit(1);
        }

        // Use reduced accuracy (no planet provider)
        let accuracy = sn::novas_accuracy_NOVAS_REDUCED_ACCURACY;

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
            "'{}' observed from lon = {:.3}, lat = {:.3}:",
            source_name.to_string_lossy(),
            obs.on_surf.longitude,
            obs.on_surf.latitude
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
