use std::os::raw::{c_ushort, c_void};
use std::ptr;
use x11::xlib::{XCloseDisplay, XDefaultScreen, XFree, XOpenDisplay, XRootWindow};
use x11::xrandr::{
    XRRAllocGamma, XRRCrtcGamma, XRRGetCrtcGammaSize, XRRGetScreenResourcesCurrent, XRRSetCrtcGamma,
};

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
        XCloseDisplay(display);
    }
}
