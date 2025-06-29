/**
 * @file
 *
 * @date Created  on Mar 6, 2025
 * @author G. Kaplan and Attila Kovacs
 *
 *  Various function for calculating the equator and equinox of date, and related quatities.
 */

#include <string.h>
#include <errno.h>

/// \cond PRIVATE
#define __NOVAS_INTERNAL_API__    ///< Use definitions meant for internal use by SuperNOVAS only
#include "novas.h"
/// \endcond

/// \cond PRIVATE

/**
 * Celestial pole offset &psi; for high-precision applications. It was visible to users in NOVAS C 3.1,
 * hence we continue to expose it also for back compatibility.
 *
 * @sa EPS_COR
 * @sa cel_pole()
 *
 * @deprecated This old way of incorporating Earth orientation parameters into the true equator
 *             and equinox is now disfavored. Instead, wobble() should be used to convert between
 *             the Terrestrial Intermediate Reference System (TIRS) / Pseudo Earth Fixed (PEF) and
 *             the International Terrestrial Reference System (ITRS) going forward.
 */
double PSI_COR = 0.0;

/**
 * Celestial pole offset &epsilon; for high-precision applications. It was visible to users in NOVAS C 3.1,
 * hence we continue to expose it also for back compatibility.
 *
 * @sa PSI_COR
 * @sa cel_pole()
 *
 * @deprecated This old way of incorporating Earth orientation parameters into the true equator
 *             and equinox is now disfavored. Instead, wobble() should be used to convert between
 *             the Terrestrial Intermediate Reference System (TIRS) / Pseudo Earth Fixed (PEF) and
 *             the International Terrestrial Reference System (ITRS) going forward.
 */
double EPS_COR = 0.0;

/// \endcond

/**
 * Specifies the unmodeled celestial pole offsets for high-precision applications to be applied to
 * the True of Date (TOD) equator, in the old, pre IAU 2006 methodology. These offsets must not
 * include tidal terms, and should be specified relative to the IAU2006 precession/nutation model
 * to provide a correction to the modeled (precessed and nutated) position of Earth's pole, such
 * those derived from observations and published by IERS.
 *
 * The call sets the global variables `PSI_COR` and `EPS_COR`, for subsequent calls to `e_tilt()`.
 * As such, it should be called to specify pole offsets prior to legacy NOVAS C equinox-specific
 * calls. The global values of `PSI_COR` and `EPS_COR` specified via this function will be
 * effective until explicitly changed again.
 *
 * NOTES:
 * <ol>
 * <li>
 *  The pole offsets et this way will affect all future TOD-based calculations, until the pole
 *  is changed or reset again. Hence, you should be extremely careful using it (if at all), as it
 *  may become an unpredictable source of inaccuracy if implicitly applied without intent to do so.
 * </li>
 * <li>
 *  The current UT1 - UTC time difference, and polar offsets, historical data and near-term
 *  projections are published in the
 *  <a href="https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html>IERS Bulletins</a>
 * </li>
 * <li>
 *  If &Delta;&delta;&psi;, &Delta;&delta;d&epsilon; offsets are specified, these must be the residual
 *  corrections relative to the IAU 2006 precession/nutation model (not the Lieske et al. 1977 model!).
 *  As such, they are just a rotated version of the newer dx, dy offsets published by IERS.
 * </li>
 * <li>
 *  The equivalent IAU 2006 standard is to apply dx, dy pole offsets only for converting
 *  between TIRS and ITRS, e.g. via `wobble()`).
 * </li>
 * <li>
 *  There is no need to define pole offsets this way when using the newer frame-based
 *  approach introduced in SuperNOVAS. If the pole offsets are specified on a per-frame basis
 *  during the initialization of each observing frame, the offsets will be applied for the
 *  TIRS / ITRS conversion only, and not to the TOD equator per se.
 * </li>
 * </ol>
 *
 * REFERENCES:
 * <ol>
 *  <li>Kaplan, G. (2005), US Naval Observatory Circular 179.</li>
 *  <li>Kaplan, G. (2003), USNO/AA Technical Note 2003-03.</li>
 * </ol>
 *
 * @param jd_tt     [day] Terrestrial Time (TT) based Julian date. Used only if 'type' is
 *                  POLE_OFFSETS_X_Y (2), to transform dx and dy to the equivalent &Delta;&delta;&psi;
 *                  and &Delta;&delta;&epsilon; values.
 * @param type      POLE_OFFSETS_DPSI_DEPS (1) if the offsets are &Delta;&delta;&psi;,
 *                  &Delta;&delta;&epsilon; relative to the IAU 20006 precession/nutation model; or
 *                  POLE_OFFSETS_X_Y (2) if they are dx, dy offsets relative to the IAU 2000/2006
 *                  precession-nutation model.
 * @param dpole1    [mas] Value of celestial pole offset in first coordinate, (&Delta;&delta;&psi; for
 *                  or dx) in milliarcseconds, relative to the IAU2006 precession/nutation model.
 * @param dpole2    [mas] Value of celestial pole offset in second coordinate, (&Delta;&delta;&epsilon;
 *                  or dy) in milliarcseconds, relative to the IAU2006 precession/nutation model.
 * @return          0 if successful, or else 1 if 'type' is invalid.
 *
 * @sa wobble()
 * @sa e_tilt()
 * @sa place()
 * @sa cirs_to_itrs()
 * @sa tod_to_itrs()
 * @sa get_ut1_to_tt()
 * @sa sidereal_time()
 * @sa NOVAS_FULL_ACCURACY
 *
 * @deprecated This old way of incorporating Earth orientation parameters into the true equator
 *             and equinox is now disfavored. Instead, wobble() should be used to convert between
 *             the Terrestrial Intermediate Reference System (TIRS) / Pseudo Earth Fixed (PEF) and
 *             the International Terrestrial Reference System (ITRS) going forward.
 */
short cel_pole(double jd_tt, enum novas_pole_offset_type type, double dpole1, double dpole2) {
  switch(type) {
    case POLE_OFFSETS_DPSI_DEPS:

      // Angular coordinates of modeled pole referred to mean ecliptic of
      // date, that is,delta-delta-psi and delta-delta-epsilon.
      PSI_COR = 1e-3 * dpole1;
      EPS_COR = 1e-3 * dpole2;
      break;

    case POLE_OFFSETS_X_Y: {
      polar_dxdy_to_dpsideps(jd_tt, dpole1, dpole2, &PSI_COR, &EPS_COR);
      break;
    }

    default:
      return novas_error(1, EINVAL, "cel_pole", "invalid polar offset type: %d", type);
  }

  return 0;
}

/// \cond PRIVATE

/**
 * Converts <i>dx,dy</i> pole offsets to d&psi; d&epsilon;. The former is in GCRS, vs the latter in
 * True of Date (TOD) -- and note the different units!
 *
 * NOTES:
 * <ol>
 * <li>The current UT1 - UTC time difference, and polar offsets, historical data and near-term
 * projections are published in the
 * <a href="https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html>IERS Bulletins</a>
 * </li>
 * </ol>
 *
 * REFERENCES:
 * <ol>
 *  <li>Kaplan, G. (2005), US Naval Observatory Circular 179.</li>
 *  <li>Kaplan, G. (2003), USNO/AA Technical Note 2003-03.</li>
 * </ol>
 *
 * @param jd_tt       [day] Terrestrial Time (TT) based Julian Date.
 * @param dx          [mas] Earth orientation: GCRS pole offset dx, e.g. as published by IERS Bulletin A.
 * @param dy          [mas] Earth orientation: GCRS pole offset dy, e.g. as published by IERS Bulletin A.
 * @param[out] dpsi   [arcsec] Calculated TOD orientation d&psi;.
 * @param[out] deps   [arcsec] Calculated TOD orientation d&epsilon;.
 * @return            0
 *
 * @sa cel_pole()
 *
 * @since 1.1
 * @author Attila Kovacs
 */
int polar_dxdy_to_dpsideps(double jd_tt, double dx, double dy, double *restrict dpsi, double *restrict deps) {
  // Components of modeled pole unit vector referred to GCRS axes, that is, dx and dy.
  const double t = (jd_tt - JD_J2000) / JULIAN_CENTURY_DAYS;

  // The following algorithm, to transform dx and dy to
  // delta-delta-psi and delta-delta-epsilon, is from eqs. (7)-(9) of the
  // second reference.
  //
  // Trivial model of pole trajectory in GCRS allows computation of dz.
  const double x = (2004.190 * t) * ARCSEC;
  const double dz = -(x + 0.5 * x * x * x) * dx;

  // Form pole offset vector (observed - modeled) in GCRS.
  double dp[3] = { dx * MAS, dy * MAS, dz * MAS };

  // Precess pole offset vector to mean equator and equinox of date.
  gcrs_to_mod(jd_tt, dp, dp);

  // Compute delta-delta-psi and delta-delta-epsilon in arcseconds.
  if(dpsi) {
    // Compute sin_e of mean obliquity of date.
    const double sin_e = sin(mean_obliq(jd_tt) * ARCSEC);
    *dpsi = (dp[0] / sin_e) / ARCSEC;
  }
  if(deps)
    *deps = dp[1] / ARCSEC;

  return 0;
}
/// \endcond

/**
 * Computes quantities related to the orientation of the Earth's rotation axis at the specified Julian
 * date.
 *
 * Unmodelled corrections to earth orientation can be defined via `cel_pole()` prior to this call.
 *
 * NOTES:
 * <ol>
 * <li>This function caches the results of the last calculation in case it may be re-used at
 * no extra computational cost for the next call.</li>
 * </ol>
 *
 * @param jd_tdb        [day] Barycentric Dynamical Time (TDB) based Julian date.
 * @param accuracy      NOVAS_FULL_ACCURACY (0) or NOVAS_REDUCED_ACCURACY (1)
 * @param[out] mobl     [deg] Mean obliquity of the ecliptic. It may be NULL if not required.
 * @param[out] tobl     [deg] True obliquity of the ecliptic. It may be NULL if not required.
 * @param[out] ee       [s] Equation of the equinoxes in seconds of time. It may be NULL if not required.
 * @param[out] dpsi     [arcsec] Nutation in longitude. It may be NULL if not required.
 * @param[out] deps     [arcsec] Nutation in obliquity. It may be NULL if not required.
 *
 * @return          0 if successful, or -1 if the accuracy argument is invalid
 *
 * @sa cel_pole()
 * @sa place()
 * @sa equ2ecl()
 * @sa ecl2equ()
 */
int e_tilt(double jd_tdb, enum novas_accuracy accuracy, double *restrict mobl, double *restrict tobl,
        double *restrict ee, double *restrict dpsi, double *restrict deps) {
  double t, d_psi = NAN, d_eps = NAN, mean_ob, true_ob, eqeq;

  if(accuracy != NOVAS_FULL_ACCURACY && accuracy != NOVAS_REDUCED_ACCURACY)
    return novas_error(-1, EINVAL, "e_tilt", "invalid accuracy: %d", accuracy);

  // Compute time in Julian centuries from epoch J2000.0.
  t = (jd_tdb - JD_J2000) / JULIAN_CENTURY_DAYS;

  nutation_angles(t, accuracy, &d_psi, &d_eps);

  d_psi += PSI_COR;
  d_eps += EPS_COR;

  // Compute mean obliquity of the ecliptic in degrees.
  mean_ob = mean_obliq(jd_tdb) / 3600.0;

  // Obtain complementary terms for equation of the equinoxes in seconds of time.
  eqeq = (d_psi * cos(mean_ob * DEGREE) + ee_ct(jd_tdb, 0.0, accuracy) / ARCSEC) / 15.0;

  // Compute true obliquity of the ecliptic in degrees.
  true_ob = mean_ob + d_eps / 3600.0;

  // Set output values.
  if(dpsi)
    *dpsi = d_psi;
  if(deps)
    *deps = d_eps;
  if(ee)
    *ee = eqeq;
  if(mobl)
    *mobl = mean_ob;
  if(tobl)
    *tobl = true_ob;

  return 0;
}

/**
 * Returns the general precession in longitude (Simon et al. 1994), equivalent to 5028.8200
 * arcsec/cy at J2000.
 *
 * @param t   [cy] Julian centuries since J2000
 * @return    [rad] the approximate precession angle [-&pi;:&pi;].
 *
 * @sa planet_lon()
 * @sa nutation_angles()
 * @sa ee_ct()
 * @sa NOVAS_JD_J2000
 *
 * @since 1.0
 * @author Attila Kovacs
 */
double accum_prec(double t) {
  // General precession in longitude (Simon et al. 1994), equivalent
  // to 5028.8200 arcsec/cy at J2000.
  return remainder((0.024380407358 + 0.000005391235 * t) * t, TWOPI);
}

/**
 * Returns the planetary longitude, for Mercury through Neptune, w.r.t. mean dynamical
 * ecliptic and equinox of J2000, with high order terms omitted (Simon et al. 1994,
 * 5.8.1-5.8.8).
 *
 * @param t       [cy] Julian centuries since J2000
 * @param planet  Novas planet id, e.g. NOVAS_MARS.
 * @return        [rad] The approximate longitude of the planet in radians [-&pi;:&pi;],
 *                or NAN if the `planet` id is out of range.
 *
 * @sa accum_prec()
 * @sa nutation_angles()
 * @sa ee_ct()
 * @sa NOVAS_JD_J2000
 *
 * @since 1.0
 * @author Attila Kovacs
 */
double planet_lon(double t, enum novas_planet planet) {
  static const double c[9][2] = {
          { 0.0, 0.0 }, //
          { 4.402608842461, 2608.790314157421 },  // Mercury
          { 3.176146696956, 1021.328554621099 },  // Venus
          { 1.753470459496,  628.307584999142 },  // Earth
          { 6.203476112911,  334.061242669982 },  // Mars
          { 0.599547105074,   52.969096264064 },  // Jupiter
          { 0.874016284019,   21.329910496032 },  // Saturn
          { 5.481293871537,    7.478159856729 },  // Uranus
          { 5.311886286677,    3.813303563778 }   // Neptune
  };

  if(planet < NOVAS_MERCURY || planet > NOVAS_NEPTUNE) {
    novas_set_errno(EINVAL, "planet_lon", "invalid planet number: %d", planet);
    return NAN;
  }

  return remainder(c[planet][0] + c[planet][1] * t, TWOPI);
}

/**
 * Computes the mean obliquity of the ecliptic.
 *
 * REFERENCES:
 * <ol>
 * <li>Capitaine et al. (2003), Astronomy and Astrophysics 412, 567-586.</li>
 * </ol>
 *
 * @param jd_tdb      [day] Barycentric Dynamic Time (TDB) based Julian date
 * @return            [arcsec] Mean obliquity of the ecliptic in arcseconds.
 *
 * @sa e_tilt()
 * @sa equ2ecl()
 * @sa ecl2equ()
 * @sa tt2tdb()
 *
 */
double mean_obliq(double jd_tdb) {
  // Compute time in Julian centuries from epoch J2000.0.
  const double t = (jd_tdb - JD_J2000) / JULIAN_CENTURY_DAYS;

  // Compute the mean obliquity in arcseconds.  Use expression from the
  // reference's eq. (39) with obliquity at J2000.0 taken from eq. (37)
  // or Table 8.
  return ((((-0.0000000434 * t - 0.000000576) * t + 0.00200340) * t - 0.0001831) * t - 46.836769) * t + 84381.406;
}

/**
 * Compute the intermediate right ascension of the equinox at the input Julian date, using an
 * analytical expression for the accumulated precession in right ascension.  For the true
 * equinox, the result is the equation of the origins.
 *
 * NOTES:
 * <ol>
 * <li>Fixes bug in NOVAS C 3.1, which returned the value for the wrong 'equinox' if
 * 'equinox = 1' was requested for the same 'jd_tbd' and 'accuracy' as a the preceding
 * call with 'equinox = 0'. As a result, the caller ended up with the mean instead
 * of the expected true equinox R.A. value.</li>
 * </ol>
 *
 * REFERENCES:
 * <ol>
 * <li>Capitaine, N. et al. (2003), Astronomy and Astrophysics 412, 567-586, eq. (42).</li>
 * </ol>
 *
 * @param jd_tdb      [day] Barycentric Dynamic Time (TDB) based Julian date
 * @param equinox     NOVAS_MEAN_EQUINOX (0) or NOVAS_TRUE_EQUINOX (1, or non-zero)
 * @param accuracy    NOVAS_FULL_ACCURACY (0) or NOVAS_REDUCED_ACCURACY (1, or non-zero)
 * @return            [h]  Intermediate right ascension of the equinox, in hours (+ or -).
 *                    If 'equinox' = 1 (i.e true equinox), then the returned value is
 *                    the equation of the origins.
 *
 * @sa cio_location()
 * @sa gcrs_to_cirs()
 *
 * @deprecated      (<i>for internal use</i>) There is no good reason why this function should
 *                  be exposed to users. It is intended only for `cio_location()` internally.
 */
double ira_equinox(double jd_tdb, enum novas_equinox_type equinox, enum novas_accuracy accuracy) {
  // Compute time in Julian centuries from J2000
  double t = (jd_tdb - JD_J2000) / JULIAN_CENTURY_DAYS;

  // Precession in RA in arcseconds taken from the reference.
  double prec_ra = 0.014506 + ((((-0.0000000368 * t - 0.000029956) * t - 0.00000044) * t + 1.3915817) * t + 4612.156534) * t;

  // arcsec -> seconds of time
  prec_ra /= 15.0;

  // For the true equinox, obtain the equation of the equinoxes in time
  // seconds, which includes the 'complementary terms'.
  if(equinox == NOVAS_TRUE_EQUINOX) {
    double eqeq = 0.0;

    // Fail-safe accuracy
    if(accuracy != NOVAS_REDUCED_ACCURACY)
      accuracy = NOVAS_FULL_ACCURACY;

    // Add equation of equinoxes.
    e_tilt(jd_tdb, accuracy, NULL, NULL, &eqeq, NULL, NULL);
    prec_ra += eqeq;
  }

  // seconds -> hours
  return -prec_ra / 3600.0;
}

/**
 * Computes the "complementary terms" of the equation of the equinoxes. The input Julian date
 * can be split into high and low order parts for improved accuracy. Typically, the split is
 * into integer and fractiona parts. If the precision of a single part is sufficient, you may
 * set the low order part to 0.
 *
 * The series used in this function was derived from the first reference.  This same series was
 * also adopted for use in the IAU's Standards of Fundamental Astronomy (SOFA) software (i.e.,
 * subroutine `eect00.for` and function `eect00.c`).
 *
 * The low-accuracy series used in this function is a simple implementation derived from the first
 * reference, in which terms smaller than 2 microarcseconds have been omitted.
 *
 * NOTES:
 * <ol>
 * <li>This function caches the results of the last calculation in case it may be re-used at
 * no extra computational cost for the next call.</li>
 * </ol>
 *
 * REFERENCES:
 * <ol>
 * <li>Capitaine, N., Wallace, P.T., and McCarthy, D.D. (2003). Astron. &amp; Astrophys. 406, p.
 * 1135-1149. Table 3.</li>
 * <li>IERS Conventions (2010), Chapter 5, p. 60, Table 5.2e.<br>
 * (Table 5.2e presented in the printed publication is a truncated
 * series. The full series, which is used in NOVAS, is available
 * on the IERS Conventions Center website:
 * <a href="ftp://tai.bipm.org/iers/conv2010/chapter5/tab5.2e.txt">
 * ftp://tai.bipm.org/iers/conv2010/chapter5/tab5.2e.txt</a>)
 * </li>
 * </ol>
 *
 * @param jd_tt_high  [day] High-order part of TT based Julian date.
 * @param jd_tt_low   [day] Low-order part of TT based Julian date.
 * @param accuracy    NOVAS_FULL_ACCURACY (0) or NOVAS_REDUCED_ACCURACY (1)
 * @return            [rad] Complementary terms, in radians.
 *
 * @sa e_tilt()
 * @sa cel_pole()
 * @sa nutation()
 * @sa sidereal_time()
 *
 * @deprecated (<i>for intrernal use</i>) There is no good reason why this function should
 *             be exposed to users of the library. It is intended only for use by `e_tilt()`
 *             internally.
 */
double ee_ct(double jd_tt_high, double jd_tt_low, enum novas_accuracy accuracy) {
  static THREAD_LOCAL double last_tt;
  static THREAD_LOCAL double last_ee;
  static THREAD_LOCAL enum novas_accuracy last_acc;

  last_tt = NAN;
  last_ee = NAN;
  last_acc = -1;

  // Argument coefficients for t^0.
  const int8_t ke0_t[33][14] = { //
          { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, -2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, -2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, -2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, 2, -2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, 2, -2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 4, -4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 1, -1, 1, 0, -8, 12, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, -2, 2, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, -2, 2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 0, 0, 0, 0, 8, -13, 0, 0, 0, 0, 0, -1 }, //
          { 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 2, 0, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, 0, -2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 1, 2, -2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, 0, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 4, -2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 0, 0, 2, -2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, -2, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0 }, //
          { 1, 0, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0 } };

  // Argument coefficients for t^1.
  //const char ke1[14] = {0,  0,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0};

  // Sine and cosine coefficients for t^0.
  const float se0_t[33][2] = { //
          { +2640.96e-6, -0.39e-6 }, //
          { +63.52e-6, -0.02e-6 }, //
          { +11.75e-6, +0.01e-6 }, //
          { +11.21e-6, +0.01e-6 }, //
          { -4.55e-6, +0.00e-6 }, //
          { +2.02e-6, +0.00e-6 }, //
          { +1.98e-6, +0.00e-6 }, //
          { -1.72e-6, +0.00e-6 }, //
          { -1.41e-6, -0.01e-6 }, //
          { -1.26e-6, -0.01e-6 }, //
          { -0.63e-6, +0.00e-6 }, //
          { -0.63e-6, +0.00e-6 }, //
          { +0.46e-6, +0.00e-6 }, //
          { +0.45e-6, +0.00e-6 }, //
          { +0.36e-6, +0.00e-6 }, //
          { -0.24e-6, -0.12e-6 }, //
          { +0.32e-6, +0.00e-6 }, //
          { +0.28e-6, +0.00e-6 }, //
          { +0.27e-6, +0.00e-6 }, //
          { +0.26e-6, +0.00e-6 }, //
          { -0.21e-6, +0.00e-6 }, //
          { +0.19e-6, +0.00e-6 }, //
          { +0.18e-6, +0.00e-6 }, //
          { -0.10e-6, +0.05e-6 }, //
          { +0.15e-6, +0.00e-6 }, //
          { -0.14e-6, +0.00e-6 }, //
          { +0.14e-6, +0.00e-6 }, //
          { -0.14e-6, +0.00e-6 }, //
          { +0.14e-6, +0.00e-6 }, //
          { +0.13e-6, +0.00e-6 }, //
          { -0.11e-6, +0.00e-6 }, //
          { +0.11e-6, +0.00e-6 }, //
          { +0.11e-6, +0.00e-6 } };

  // Sine and cosine coefficients for t^1.
  const double se1[2] = { -0.87e-6, +0.00e-6 };

  novas_delaunay_args fa2;
  double fa[14];

  // Interval between fundamental epoch J2000.0 and current date.
  double t;

  if(accuracy != NOVAS_FULL_ACCURACY)
    accuracy = NOVAS_REDUCED_ACCURACY;

  if(novas_time_equals(jd_tt_high + jd_tt_low, last_tt) && accuracy == last_acc)
    return last_ee;

  last_tt = jd_tt_high + jd_tt_low;
  last_acc = accuracy;

  t = ((jd_tt_high - JD_J2000) + jd_tt_low) / JULIAN_CENTURY_DAYS;

  // High accuracy mode.
  if(accuracy == NOVAS_FULL_ACCURACY) {
    double s0 = 0.0, s1 = 0.0;
    int i;

    // Fill the 5 Earth-Sun-Moon fundamental args
    fund_args(t, (novas_delaunay_args*) fa);

    // Add planet longitudes
    for(i = NOVAS_MERCURY; i <= NOVAS_NEPTUNE; i++) {
      int j = i - NOVAS_MERCURY;
      fa[5 + j] = planet_lon(t, i);
    }

    // General accumulated precession longitude
    fa[13] = accum_prec(t);

    // Evaluate the complementary terms.
    for(i = 33; --i >= 0;) {
      const int8_t *ke = &ke0_t[i][0];
      const float *se = &se0_t[i][0];

      double a = 0.0;
      int j;

      for(j = 14; --j >= 0;)
        if(ke[j])
          a += ke[j] * fa[j];

      s0 += se[0] * sin(a);
      if(se[1])
        s0 += se[1] * cos(a);
    }

    // AK: Skip 0 terms from ke1[]
    //
    // a = 0.0;
    // for(j = 0; j < 14; j++) a += (double) (ke1[j]) * fa[j];
    s1 += se1[0] * sin(fa[4]);

    last_ee = (s0 + s1 * t) * ARCSEC;
  }
  else {
    // Low accuracy mode: Terms smaller than 2 microarcseconds omitted
    fund_args(t, &fa2);

    last_ee = (2640.96e-6 * sin(fa2.Omega) //
    + 63.52e-6 * sin(2.0 * fa2.Omega) //
    + 11.75e-6 * sin(2.0 * fa2.F - 2.0 * fa2.D + 3.0 * fa2.Omega) //
    + 11.21e-6 * sin(2.0 * fa2.F - 2.0 * fa2.D + fa2.Omega) //
    - 4.55e-6 * sin(2.0 * fa2.F - 2.0 * fa2.D + 2.0 * fa2.Omega) //
    + 2.02e-6 * sin(2.0 * fa2.F + 3.0 * fa2.Omega) //
    + 1.98e-6 * sin(2.0 * fa2.F + fa2.Omega) //
    - 1.72e-6 * sin(3.0 * fa2.Omega) //
    - 0.87e-6 * t * sin(fa2.Omega) //
    ) * ARCSEC;
  }

  return last_ee;
}

/**
 * Compute the fundamental arguments (mean elements) of the Sun and Moon.
 *
 * REFERENCES:
 * <ol>
 * <li>Simon et al. (1994) Astronomy and Astrophysics 282, 663-683, esp. Sections 3.4-3.5.</li>
 * </ol>
 *
 * @param t       [cy] TDB time in Julian centuries since J2000.0
 * @param[out] a  [rad] Fundamental arguments data to populate (5 doubles) [0:2&pi;]
 *
 * @return        0 if successful, or -1 if the output pointer argument is NULL.
 *
 * @sa nutation_angles()
 * @sa ee_ct()
 * @sa NOVAS_JD_J2000
 */
int fund_args(double t, novas_delaunay_args *restrict a) {
  if(!a)
    return novas_error(-1, EINVAL, "fund_args", "NULL output pointer");

  a->l = 485868.249036 + t * (1717915923.2178 + t * (31.8792 + t * (0.051635 + t * (-0.00024470))));
  a->l1 = 1287104.793048 + t * (129596581.0481 + t * (-0.5532 + t * (0.000136 + t * (-0.00001149))));
  a->F = 335779.526232 + t * (1739527262.8478 + t * (-12.7512 + t * (-0.001037 + t * (0.00000417))));
  a->D = 1072260.703692 + t * (1602961601.2090 + t * (-6.3706 + t * (0.006593 + t * (-0.00003169))));
  a->Omega = 450160.398036 + t * (-6962890.5431 + t * (7.4722 + t * (0.007702 + t * (-0.00005939))));

  /*
  // From Chapront, J. et al., 2002, A&A 387, 700–709
  a->l =       485868.2264 + t * (1717915923.0024 + t * ( 31.3939 + t * ( 0.051651 - t * 0.00024470)));
  a->l1 =     1287104.7744 + t * ( 129596581.0733 + t * ( -0.5529 + t * ( 0.000147 + t * 0.00000015)));
  a->F =       335779.5517 + t * (1739527263.2179 + t * (-13.2293 + t * (-0.001021 + t * 0.00000417)));
  a->D =      1072260.6902 + t * (1602961601.0312 + t * ( -6.8498 + t * ( 0.006595 - t * 0.00003184)));
  a->Omega =   450160.3265 + t * (  -6967919.8851 + t * (  6.3593 + t * ( 0.007625 - t * 0.00003586)));
  */

  a->l = novas_norm_ang(a->l * ARCSEC);
  a->l1 = novas_norm_ang(a->l1 * ARCSEC);
  a->F = novas_norm_ang(a->F * ARCSEC);
  a->D = novas_norm_ang(a->D * ARCSEC);
  a->Omega = novas_norm_ang(a->Omega * ARCSEC);

  return 0;
}

/**
 * Precesses equatorial rectangular coordinates from one epoch to another. Unlike the original
 * NOVAS routine, this routine works for any pairing of the time arguments.
 *
 * This function calculates precession for the old (pre IAU 2000) methodology. Its main use
 * for NOVAS users is to allow converting older catalog coordinates e.g. to J2000 coordinates,
 * which then can be converted to the now standard ICRS system via frame_tie().
 *
 * NOTE:
 * <ol>
 * <li>Unlike the original NOVAS C 3.1 version, this one does not require that one
 *     of the time arguments must be J2000. You can precess from any date to
 *     any other date, and the intermediate epoch of J2000 will be handled internally
 *     as needed.</li>
 *
 * <li>This function caches the results of the last calculation in case it may be re-used at
 *     no extra computational cost for the next call.</li>
 * </ol>
 *
 * REFERENCES:
 * <ol>
 * <li>Explanatory Supplement To The Astronomical Almanac, pp. 103-104.</li>
 * <li>Capitaine, N. et al. (2003), Astronomy And Astrophysics 412, pp. 567-586.</li>
 * <li>Hilton, J. L. et al. (2006), IAU WG report, Celest. Mech., 94, pp. 351-367.</li>
 * <li>Capitaine, N., P.T. Wallace and J. Chapront (2005), “Improvement of the IAU 2000 precession
 *     model.” Astronomy &amp; Astrophysics, Vol. 432, pp. 355–67.</li>
 * <li>Liu, J.-C., &amp Capitaine, N. (2017), A&A 597, A83</li>
 * </ol>
 *
 * @param jd_tdb_in   [day] Barycentric Dynamic Time (TDB) based Julian date of the input
 *                    epoch
 * @param in          Position 3-vector, geocentric equatorial rectangular coordinates,
 *                    referred to mean dynamical equator and equinox of the initial epoch.
 * @param jd_tdb_out  [day] Barycentric Dynamic Time (TDB) based Julian date of the output
 *                    epoch
 * @param[out] out    Position 3-vector, geocentric equatorial rectangular coordinates,
 *                    referred to mean dynamical equator and equinox of the final epoch.
 *                    It can be the same vector as the input.
 * @return            0 if successful, or -1 if either of the position vectors is NULL.
 *
 * @sa nutation()
 * @sa frame_tie()
 * @sa novas_epoch()
 * @sa tt2tdb()
 * @sa cio_basis()
 * @sa NOVAS_TOD
 * @sa NOVAS_JD_J2000
 * @sa NOVAS_JD_B1950
 * @sa NOVAS_JD_B1900
 */
short precession(double jd_tdb_in, const double *in, double jd_tdb_out, double *out) {
  static THREAD_LOCAL double djd_last[2];
  djd_last[0] = NAN;
  djd_last[1] = NAN;
  static THREAD_LOCAL double xx[2], yx[2], zx[2], xy[2], yy[2], zy[2], xz[2], yz[2], zz[2];

  double t;
  int i = 0;

  if(!in || !out)
    return novas_error(-1, EINVAL, "precession", "NULL input or output 3-vector: in=%p, out=%p", in, out);

  if(jd_tdb_in == jd_tdb_out) {
    if(out != in)
      memcpy(out, in, XYZ_VECTOR_SIZE);
    return 0;
  }

  // Check to be sure that either 'jd_tdb1' or 'jd_tdb2' is equal to JD_J2000.
  if(!novas_time_equals(jd_tdb_in, JD_J2000) && !novas_time_equals(jd_tdb_out, JD_J2000)) {
    // Do the precession in two steps...
    precession(jd_tdb_in, in, JD_J2000, out);
    precession(JD_J2000, out, jd_tdb_out, out);
    return 0;
  }

  // 't' is time in TDB centuries between the two epochs.
  t = (jd_tdb_out - jd_tdb_in);
  if(jd_tdb_out == JD_J2000) {
    t = -t;
    i = 1;
  }

  if(!novas_time_equals(t, djd_last[i])) {
    double psia, omegaa, chia, sa, ca, sb, cb, sc, cc, sd, cd, t1, t2;
    double eps0 = 84381.406;

    djd_last[i] = t;

    // Now change t to Julian centuries
    t /= JULIAN_CENTURY_DAYS;

    // Numerical coefficients of psi_a, omega_a, and chi_a, along with
    // epsilon_0, the obliquity at J2000.0, are 4-angle formulation from
    // Capitaine et al. (2003), eqs. (4), (37), & (39).
    psia = ((((-0.0000000951 * t + 0.000132851) * t - 0.00114045) * t - 1.0790069) * t + 5038.481507) * t;
    omegaa = ((((+0.0000003337 * t - 0.000000467) * t - 0.00772503) * t + 0.0512623) * t - 0.025754) * t + eps0;
    chia = ((((-0.0000000560 * t + 0.000170663) * t - 0.00121197) * t - 2.3814292) * t + 10.556403) * t;

    // P03rev2 / Capitaine at al. (2005) eqs. (11)
    //psia = t * (5038.482090 + t * (-1.0789921 + t * (-0.00114040 + t * (0.000132851 - t * 0.0000000951))));
    //omegaa = eps0 + t * (-0.025675 + t * (0.0512622 + t * (-0.00772501 + t * (-0.000000467 + t * 0.0000003337))));

    // Liu & Capitaine (2017)
    //chia = t * (10.556240 + t * (-2.3813876 + t * (-0.00121311 + t * (0.000160286 + t * 0.000000086))));
    //psia = t * (5038.481270 + t * (-1.0732468 + t * (0.01573403 + t * (0.000127135 - t * 0.0000001020))));
    //omegaa = eps0 + t * (-0.024725 + t * (0.0512626 + t * (-0.0077249 + t * (-0.000000267 + t * 0.000000267))));

    eps0 *= ARCSEC;
    psia *= ARCSEC;
    omegaa *= ARCSEC;
    chia *= ARCSEC;

    sa = sin(eps0);
    ca = cos(eps0);
    sb = sin(-psia);
    cb = cos(-psia);
    sc = sin(-omegaa);
    cc = cos(-omegaa);
    sd = sin(chia);
    cd = cos(chia);

    // Compute elements of precession rotation matrix equivalent to
    // R3(chi_a) R1(-omega_a) R3(-psi_a) R1(epsilon_0).
    t1 = cd * sb + sd * cc * cb;
    t2 = sd * sc;
    xx[i] = cd * cb - sb * sd * cc;
    yx[i] = ca * t1 - sa * t2;
    zx[i] = sa * t1 + ca * t2;

    t1 = cd * cc * cb - sd * sb;
    t2 = cd * sc;
    xy[i] = -sd * cb - sb * cd * cc;
    yy[i] = ca * t1 - sa * t2;
    zy[i] = sa * t1 + ca * t2;

    xz[i] = sb * sc;
    yz[i] = -sc * cb * ca - sa * cc;
    zz[i] = -sc * cb * sa + cc * ca;
  }

  if(jd_tdb_out == JD_J2000) {
    const double x = in[0], y = in[1], z = in[2];
    // Perform rotation from epoch to J2000.0.
    out[0] = xx[1] * x + xy[1] * y + xz[1] * z;
    out[1] = yx[1] * x + yy[1] * y + yz[1] * z;
    out[2] = zx[1] * x + zy[1] * y + zz[1] * z;
  }
  else {
    const double x = in[0], y = in[1], z = in[2];
    // Perform rotation from J2000.0 to epoch.
    out[0] = xx[0] * x + yx[0] * y + zx[0] * z;
    out[1] = xy[0] * x + yy[0] * y + zy[0] * z;
    out[2] = xz[0] * x + yz[0] * y + zz[0] * z;
  }

  return 0;
}

/**
 * Nutates equatorial rectangular coordinates from mean equator and equinox of epoch to true
 * equator and equinox of epoch. Inverse transformation may be applied by setting flag
 * 'direction'.
 *
 * This is the old (pre IAU 2006) method of nutation calculation. If you follow the now
 * standard IAU 2000/2006 methodology you will want to use nutation_angles() instead.
 *
 * REFERENCES:
 * <ol>
 * <li>Explanatory Supplement To The Astronomical Almanac, pp. 114-115.</li>
 * </ol>
 *
 *
 * @param jd_tdb      [day] Barycentric Dynamic Time (TDB) based Julian date
 * @param direction   NUTATE_MEAN_TO_TRUE (0) or NUTATE_TRUE_TO_MEAN (-1; or non-zero)
 * @param accuracy    NOVAS_FULL_ACCURACY (0) or NOVAS_REDUCED_ACCURACY (1)
 * @param in          Position 3-vector, geocentric equatorial rectangular coordinates,
 *                    referred to mean equator and equinox of epoch.
 * @param[out] out    Position vector, geocentric equatorial rectangular coordinates,
 *                    referred to true equator and equinox of epoch. It can be the same
 *                    as the input position.
 *
 * @return            0 if successful, or -1 if one of the vector arguments is NULL.
 *
 * @sa nutation_angles()
 * @sa tt2tdb()
 * @sa NOVAS_TOD
 */
int nutation(double jd_tdb, enum novas_nutation_direction direction, enum novas_accuracy accuracy, const double *in, double *out) {
  double oblm, oblt, psi;
  double cm, sm, ct, st, cp, sp;
  double xx, yx, zx, xy, yy, zy, xz, yz, zz;

  if(!in || !out)
    return novas_error(-1, EINVAL, "nutation", "NULL input or output 3-vector: in=%p, out=%p", in, out);

  // Call 'e_tilt' to get the obliquity and nutation angles.
  prop_error("nutation", e_tilt(jd_tdb, accuracy, &oblm, &oblt, NULL, &psi, NULL), 0);

  oblm *= DEGREE;
  oblt *= DEGREE;
  psi *= ARCSEC;

  cm = cos(oblm);
  sm = sin(oblm);
  ct = cos(oblt);
  st = sin(oblt);
  cp = cos(psi);
  sp = sin(psi);

  // Nutation rotation matrix follows.
  xx = cp;
  yx = -sp * cm;
  zx = -sp * sm;
  xy = sp * ct;
  yy = cp * cm * ct + sm * st;
  zy = cp * sm * ct - cm * st;
  xz = sp * st;
  yz = cp * cm * st - sm * ct;
  zz = cp * sm * st + cm * ct;

  if(direction == NUTATE_MEAN_TO_TRUE) {
    const double x = in[0], y = in[1], z = in[2];
    // Perform rotation.
    out[0] = xx * x + yx * y + zx * z;
    out[1] = xy * x + yy * y + zy * z;
    out[2] = xz * x + yz * y + zz * z;
  }
  else {
    const double x = in[0], y = in[1], z = in[2];
    // Perform inverse rotation.
    out[0] = xx * x + xy * y + xz * z;
    out[1] = yx * x + yy * y + yz * z;
    out[2] = zx * x + zy * y + zz * z;
  }

  return 0;
}


