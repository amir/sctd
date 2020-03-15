extern crate chrono;
extern crate spa;

use chrono::prelude::*;
use spa::{SunriseAndSet, calc_sunrise_and_set};
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

fn get_temp(utc: DateTime<Utc>, ss: &SunriseAndSet) -> u32 {
    let low_temp = 2500;
    let high_temp = 5500;

    match *ss {
        SunriseAndSet::Daylight(sunrise, sunset) => {
            let since_sunrise = utc.signed_duration_since(sunrise);
            let since_sunset = utc.signed_duration_since(sunset);

            // this is where gradual increase/decrease should happen
            if since_sunrise.num_seconds() < 0 {
                return low_temp;
            } else if since_sunset.num_seconds() < 0 {
                return high_temp;
            } else {
                return low_temp;
            }
        },
        SunriseAndSet::PolarDay => return high_temp,
        SunriseAndSet::PolarNight => return low_temp,
    }
}

fn main() {
    // Dublin
    let lat: f64 = 53.3498;
    let lon: f64 = 6.2603;

    loop {
        let utc: DateTime<Utc> = Utc::now();
        match calc_sunrise_and_set(utc, lat, lon) {
            Ok(ss) => set_temp(get_temp(utc, &ss)),
            Err(e) => println!("Error calculating sunrise and sunset: {:?}", e),
        }
        thread::sleep(Duration::from_secs(300));
    }
}
