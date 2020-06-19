#[macro_use]
extern crate log;

extern crate chrono;
extern crate clap;
extern crate spa;

use chrono::prelude::*;
use clap::{value_t_or_exit, App, Arg};
use env_logger::Env;
use spa::calc_sunrise_and_set;
use std::thread;
use std::time::Duration;

fn main() {
    let env = Env::default().filter_or("SCTD_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let matches = App::new("sctd")
        .version("0.2.0")
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
        let mut temp = 0;

        loop {
            let utc: DateTime<Utc> = Utc::now();
            match calc_sunrise_and_set(utc, latitude, longitude) {
                Ok(ss) => {
                    let new_temp = sctd::get_temp(utc, &ss, latitude, longitude) as u32;
                    if new_temp != temp {
                        temp = new_temp;
                        info!("setting temprature to {}", temp);
                        sctd::set_temp(temp);
                    } else {
                        debug!("skipping temperature change as it hasn't changed ({})", temp);
                    }
                }
                Err(e) => {
                    error!(
                        "error calculating sunrise and sunset for {}, {}: {:?}",
                        latitude, longitude, e
                    );
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    }
}
