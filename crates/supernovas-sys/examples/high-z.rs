use std::ffi::CString;
use std::mem::MaybeUninit;
use std::time::{SystemTime, UNIX_EPOCH};
use supernovas_sys as sn;

// Example: High-z object position calculation using supernovas-sys (sn) in Rust


const LEAP_SECONDS: i32 = 37;      // [s] current leap seconds from IERS Bulletin C
const DUT1: f64 = 0.114;           // [s] current UT1 - UTC time difference from IERS Bulletin A
const POLAR_DX: f64 = 230.0;       // [mas] Earth polar offset x
const POLAR_DY: f64 = -62.0;       // [mas] Earth polar offset y

fn main() {
    unsafe {
        // Enable debugging
        sn::novas_debug(sn::novas_debug_mode_NOVAS_DEBUG_ON);

        // 3c273: 12h29m6.6997s +2d3m8.598s (ICRS), z=0.158339
        let ra0 = sn::novas_str_hours(CString::new("12h29m6.6997s").unwrap().as_ptr());
        let dec0 = sn::novas_str_degrees(CString::new("+2d3m8.598s").unwrap().as_ptr());

        // Define high-z source
        let mut source = MaybeUninit::<sn::object>::uninit();
        let name = CString::new("3c273").unwrap();
        let frame = CString::new("ICRS").unwrap();
        if sn::make_redshifted_object_sys(
            name.as_ptr(),
            ra0,
            dec0,
            frame.as_ptr(),
            0.158339,
            source.as_mut_ptr(),
        ) != 0
        {
            eprintln!("ERROR! defining cat_entry.");
            std::process::exit(1);
        }
        let mut source = source.assume_init();

        // Define observer on surface
        let mut obs = MaybeUninit::<sn::observer>::uninit();
        if sn::make_observer_on_surface(
            50.7374,
            7.0982,
            60.0,
            0.0,
            0.0,
            obs.as_mut_ptr(),
        ) != 0
        {
            eprintln!("ERROR! defining Earth-based observer location.");
            std::process::exit(1);
        }
        let mut obs = obs.assume_init();

        // Set astrometric time of observation
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let unix_sec = now.as_secs() as i64;
        let unix_nsec = now.subsec_nanos() as i32;

        let mut obs_time = MaybeUninit::<sn::novas_timespec>::uninit();
        if sn::novas_set_unix_time(
            unix_sec,
            unix_nsec,
            LEAP_SECONDS,
            DUT1,
            obs_time.as_mut_ptr(),
        ) != 0
        {
            eprintln!("ERROR! failed to set time of observation.");
            std::process::exit(1);
        }
        let mut obs_time = obs_time.assume_init();

        // Use reduced accuracy (no planet provider)
        let accuracy = sn::novas_accuracy_NOVAS_REDUCED_ACCURACY;

        // Initialize observing frame
        let mut obs_frame = MaybeUninit::<sn::novas_frame>::uninit();
        if sn::novas_make_frame(
            accuracy,
            &mut obs,
            &mut obs_time,
            POLAR_DX,
            POLAR_DY,
            obs_frame.as_mut_ptr(),
        ) != 0
        {
            eprintln!("ERROR! failed to define observing frame.");
            std::process::exit(1);
        }
        let mut obs_frame = obs_frame.assume_init();

        // Calculate apparent position (CIRS)
        let mut apparent = MaybeUninit::<sn::sky_pos>::uninit();
        if sn::novas_sky_pos(
            &mut source,
            &mut obs_frame,
            sn::novas_reference_system_NOVAS_CIRS,
            apparent.as_mut_ptr(),
        ) != 0
        {
            eprintln!("ERROR! failed to calculate apparent position.");
            std::process::exit(1);
        }
        let apparent = apparent.assume_init();

        println!(
            " RA = {:.9} h, Dec = {:.9} deg, z_obs = {:.9}",
            apparent.ra,
            apparent.dec,
            sn::novas_v2z(apparent.rv)
        );

        // Convert to horizontal coordinates
        let mut az = 0.0f64;
        let mut el = 0.0f64;
        if sn::novas_app_to_hor(
            &mut obs_frame,
            sn::novas_reference_system_NOVAS_CIRS,
            apparent.ra,
            apparent.dec,
            Some(sn::novas_standard_refraction),
            &mut az,
            &mut el,
        ) != 0
        {
            eprintln!("ERROR! failed to calculate azimuth / elevation.");
            std::process::exit(1);
        }

        println!(" Az = {:.6} deg, El = {:.6} deg", az, el);
    }
}
