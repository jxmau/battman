use log::{trace, error};
use std::process::Command;
use battman::batt::Battery;


fn main() {
env_logger::init();
let mut bat = match Battery::new(){
    Ok(s) => s,
    Err(e) => {
        error!("Failed to get the Battery's information.");
        error!("{e}");
        std::process::exit(1)
    }
};
cycle(&mut bat);

}


pub fn cycle(f: &mut Battery) {
            // Indicator if a notif has been already sent
            let mut full = false; // 99%
            let mut low = false;  // 15%
            let mut crit = false; // 5%


        loop {

            use std::{thread, time};

            thread::sleep(time::Duration::from_millis(250));

            f.update();

            if f.charge == true {
                if f.pct >= 99.0 && full == false {    
                    trace!("Battery considered as full | Sending Notification");
                    match Command::new("notify-send").arg("Battery | Fully Recharged").arg("Your battery is now full.").spawn() {
                        Ok(_) => trace!("Notification sent"),
                        Err(e) => {
                            error!("Failed to send the notification.");
                            error!("{e}")
                        }
                    };
                    full = true;
                }
                else if f.pct >= 15.0 && low == true { low = false; }
                else if f.pct >= 5.0 && crit == true { crit = false; }
            } else {
                let string_pct = format!("Your battery is at {}%", f.pct);
                if f.pct <= 15.0 && crit == false {
                    trace!("Battery level is considered as low | Sending Notification");
                    match Command::new("notify-send").arg("Battery | Low Level").arg(string_pct).spawn(){
                        Ok(_) => trace!("Notification sent"),
                        Err(e) => {
                            error!("Failed to send the notification.");
                            error!("{e}")
                        }
                    };
                    crit = true;
                } else if f.pct <= 5.0 && low == false {
                    trace!("Battery level is considered critical | Sending Notification");
                    match Command::new("notify-send").args(["-u", "critical"]).arg("Battery | Critical Level").arg(string_pct).spawn() {
                        Ok(_) => trace!("Notification sent"),
                        Err(e) => {
                            error!("Failed to send the notification.");
                            error!("{e}")
                        }
                    };
                    low = true;
                } else if full == true {
                    full = false;
                }
            }

        }

    }
