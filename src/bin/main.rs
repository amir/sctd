extern crate chrono;
extern crate clap;
extern crate spa;

use chrono::prelude::*;
use clap::{value_t_or_exit, App, Arg};
use spa::calc_sunrise_and_set;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("sctd")
        .version("0.1.1")
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
        sctd::reset_temp();
    } else {
        let latitude = value_t_or_exit!(matches, "latitude", f64);
        let longitude = value_t_or_exit!(matches, "longitude", f64);

        loop {
            let utc: DateTime<Utc> = Utc::now();
            match calc_sunrise_and_set(utc, latitude, longitude) {
                Ok(ss) => {
                    let temp = sctd::get_temp(utc, &ss, latitude, longitude) as u32;
                    println!("setting temprature to: {}", temp);
                    sctd::set_temp(temp);
                }
                Err(e) => {
                    println!("Error calculating sunrise and sunset: {:?}", e);
                }
            }
            thread::sleep(Duration::from_secs(300));
        }
    }
}
