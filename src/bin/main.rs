extern crate chrono;
extern crate clap;
extern crate spa;

use chrono::prelude::*;
use clap::{value_t_or_exit, App, Arg};
use spa::{calc_solar_position, calc_sunrise_and_set, SolarPos, SunriseAndSet};
use std::os::raw::{c_ushort, c_void};
use std::ptr;
use std::thread;
use std::time::Duration;
use x11::xlib::{XDefaultScreen, XFree, XOpenDisplay, XRootWindow};
use x11::xrandr::{
    XRRAllocGamma, XRRCrtcGamma, XRRGetCrtcGammaSize, XRRGetScreenResourcesCurrent, XRRSetCrtcGamma,
};

fn set_temp(temp: u32) {
    let ratio: f64 = (temp % 500) as f64 / 500f64;
    unsafe {
        let display = XOpenDisplay(ptr::null_mut());
        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        let resource = XRRGetScreenResourcesCurrent(display, root);

        for x in 0..(*resource).ncrtc {
            let crtcxid = (*resource).crtcs.offset(x as isize);
            let size = XRRGetCrtcGammaSize(display, *crtcxid);
            let crtc_gamma: *mut XRRCrtcGamma = XRRAllocGamma(size);
            let gamma = sctd::avg(temp, ratio);

            for i in 0..size {
                let g = (65535f64 * i as f64) / size as f64;
                *((*crtc_gamma).red as *mut c_ushort).offset(i as isize) = (g * gamma.red) as u16;
                *((*crtc_gamma).green as *mut c_ushort).offset(i as isize) =
                    (g * gamma.green) as u16;
                *((*crtc_gamma).blue as *mut c_ushort).offset(i as isize) = (g * gamma.blue) as u16;
            }
            XRRSetCrtcGamma(display, *crtcxid, crtc_gamma);
            XFree(crtc_gamma as *mut c_void);
        }
    }
}

fn get_transition_progress_from_elevation(elevation: f64) -> f64 {
    if elevation < -6.0 {
        return 0.0
    } else if elevation < 3.0 {
        (-6.0 - elevation) / (-6.0 - 3.0)
    } else {
        return 1.0
    }
}

fn get_temp(utc: DateTime<Utc>, ss: &SunriseAndSet, lat: f64, lon: f64) -> f64 {
    let low_temp = 3500f64;
    let high_temp = 5500f64;

    match *ss {
        SunriseAndSet::Daylight(_, _) => {
            let elevation = 90f64 - calc_solar_position(utc, lat, lon).unwrap().zenith_angle;
            let progress = get_transition_progress_from_elevation(elevation);
            low_temp + (progress * (high_temp - low_temp))
        }
        SunriseAndSet::PolarDay => high_temp,
        SunriseAndSet::PolarNight => low_temp,
    }
}

fn main() {
    let matches = App::new("sctd")
        .about("set color temperature daemon")
        .arg(
            Arg::with_name("latitude")
                .long("latitude")
                .takes_value(true)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("longitude")
                .long("longitude")
                .takes_value(true)
                .allow_hyphen_values(true),
        )
        .arg(Arg::with_name("reset").long("reset"))
        .get_matches();

    if matches.is_present("reset") {
        set_temp(5500);
    } else {
        let latitude = value_t_or_exit!(matches, "latitude", f64);
        let longitude = value_t_or_exit!(matches, "longitude", f64);

        loop {
            let utc: DateTime<Utc> = Utc::now();
            match calc_sunrise_and_set(utc, latitude, longitude) {
                Ok(ss) => set_temp(get_temp(utc, &ss, latitude, longitude) as u32),
                Err(e) => println!("Error calculating sunrise and sunset: {:?}", e),
            }
            thread::sleep(Duration::from_secs(300));
        }
    }
}
