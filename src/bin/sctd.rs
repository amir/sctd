#[macro_use]
extern crate log;

extern crate chrono;
extern crate clap;
extern crate spa;

use chrono::prelude::*;
use clap::{Arg, Command};
use env_logger::Env;
use spa::{sunrise_and_set, StdFloatOps};
use std::thread;
use std::time::Duration;

fn main() {
    let env = Env::default().filter_or("SCTD_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let matches = Command::new("sctd")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("N/A"))
        .about("set color temperature daemon")
        .arg(
            Arg::new("latitude")
                .long("latitude")
                .value_name("LATITUDE")
                .help("Latitude coordinate")
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::new("longitude")
                .long("longitude")
                .value_name("LONGITUDE")
                .help("Longitude coordinate")
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::new("reset")
                .long("reset")
                .help("Reset temperature")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("reset") {
        sctd::reset_temp();
    } else {
        let latitude: f64 = matches
            .get_one::<String>("latitude")
            .expect("latitude is required")
            .parse()
            .expect("latitude must be a valid number");
        let longitude: f64 = matches
            .get_one::<String>("longitude")
            .expect("longitude is required")
            .parse()
            .expect("longitude must be a valid number");
        let mut temp = 0;

        loop {
            let utc: DateTime<Utc> = Utc::now();
            match sunrise_and_set::<StdFloatOps>(utc, latitude, longitude) {
                Ok(ss) => {
                    let new_temp = sctd::get_temp(utc, &ss, latitude, longitude) as u32;
                    if new_temp != temp {
                        temp = new_temp;
                        info!("setting temperature to {temp}");
                        sctd::set_temp(temp);
                    } else {
                        debug!("skipping temperature change as it hasn't changed ({temp})");
                    }
                }
                Err(e) => {
                    error!(
                        "error calculating sunrise and sunset for {latitude}, {longitude}: {e:?}"
                    );
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    }
}
