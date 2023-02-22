pub mod batt;
use log::error;
use battery::units::energy::watt_hour;



pub fn get_bat_info() -> Result<(f32, bool), String> {

	    // Get manager
    	let manager = match ::battery::Manager::new() {
        Ok(s) => s,
        Err(e) => {
            error!("{e}");
            return Err("Couldn't get the manager.".to_string())
        }
    };

    // Get batteries
    let batteries = match manager.batteries() {
        Ok(s) => s,
        Err(e) => {
            error!("{e}");
            return Err("Couldn't get the iterator of the batteries".to_string())
        }
    };

    match batteries.enumerate().nth(0) {
        None => Err("Couldn't get the first battery in the iterator.".to_string()),
        Some(battery) => {
            let battery = battery.1.unwrap();
            let full = battery.energy_full().get::<watt_hour>();
            let charge = battery.energy().get::<watt_hour>();
            let pct = (( charge / full ) * 100.0).round();
            Ok((
        		pct, 
        		if battery.state() == battery::State::Charging { true } else {false},	
            ))            
        }
    }
}