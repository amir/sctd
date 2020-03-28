extern crate spa;

use chrono::prelude::*;
use spa::{calc_solar_position, SunriseAndSet};
use std::os::raw::{c_ushort, c_void};
use std::ptr;
use x11::xlib::{XDefaultScreen, XFree, XOpenDisplay, XRootWindow};
use x11::xrandr::{
    XRRAllocGamma, XRRCrtcGamma, XRRGetCrtcGammaSize, XRRGetScreenResourcesCurrent, XRRSetCrtcGamma,
};

const LOW_TEMP: f64 = 3500f64;
const HIGH_TEMP: f64 = 5500f64;

const TRANSITION_LOW: f64 = -6.0;
const TRANSITION_HIGH: f64 = 3.0;

#[derive(Debug)]
pub struct WhitePoint {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn set_temp(temp: u32) {
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
            let gamma = avg(temp, ratio);

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

pub fn reset_temp() {
    set_temp(HIGH_TEMP as u32);
}

fn get_transition_progress_from_elevation(elevation: f64) -> f64 {
    if elevation < TRANSITION_LOW {
        return 0.0;
    } else if elevation < TRANSITION_HIGH {
        (-TRANSITION_LOW - elevation) / (-TRANSITION_LOW - TRANSITION_HIGH)
    } else {
        return 1.0;
    }
}

pub fn get_temp(utc: DateTime<Utc>, ss: &SunriseAndSet, lat: f64, lon: f64) -> f64 {
    match *ss {
        SunriseAndSet::Daylight(_, _) => {
            let elevation = 90f64 - calc_solar_position(utc, lat, lon).unwrap().zenith_angle;
            let progress = get_transition_progress_from_elevation(elevation);
            LOW_TEMP + (progress * (HIGH_TEMP - LOW_TEMP))
        }
        SunriseAndSet::PolarDay => HIGH_TEMP,
        SunriseAndSet::PolarNight => LOW_TEMP,
    }
}

pub fn avg(temp: u32, ratio: f64) -> WhitePoint {
    const WPS: [WhitePoint; 20] = [
        WhitePoint {
            red: 1.00000000,
            green: 0.18172716,
            blue: 0.00000000,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.42322816,
            blue: 0.00000000,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.54360078,
            blue: 0.08679949,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.64373109,
            blue: 0.28819679,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.71976951,
            blue: 0.42860152,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.77987699,
            blue: 0.54642268,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.82854786,
            blue: 0.64816570,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.86860704,
            blue: 0.73688797,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.90198230,
            blue: 0.81465502,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.93853986,
            blue: 0.88130458,
        },
        WhitePoint {
            red: 1.00000000,
            green: 0.97107439,
            blue: 0.94305985,
        },
        WhitePoint {
            red: 1.00000000,
            green: 1.00000000,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.95160805,
            green: 0.96983355,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.91194747,
            green: 0.94470005,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.87906581,
            green: 0.92357340,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.85139976,
            green: 0.90559011,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.82782969,
            green: 0.89011714,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.80753191,
            green: 0.87667891,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.78988728,
            green: 0.86491137,
            blue: 1.00000000,
        },
        WhitePoint {
            red: 0.77442176,
            green: 0.85453121,
            blue: 1.00000000,
        },
    ];

    WhitePoint {
        red: WPS[(temp / 500) as usize].red * (1f64 - ratio)
            + WPS[(temp / 500 + 1) as usize].red * ratio,
        green: WPS[(temp / 500) as usize].green * (1f64 - ratio)
            + WPS[(temp / 500 + 1) as usize].green * ratio,
        blue: WPS[(temp / 500) as usize].blue * (1f64 - ratio)
            + WPS[(temp / 500 + 1) as usize].blue * ratio,
    }
}
