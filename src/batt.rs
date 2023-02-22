
#[derive(Debug)]
pub struct Battery {
	/// Percentage of the battery
	pub pct: f32,
	/// Is the battery charging?
	pub charge: bool,

    pub was_charging: bool
}



impl Battery {
	pub fn new() -> Result<Self, String> {

    let info = crate::get_bat_info()?;
    Ok(Self {
        pct: info.0, 
        charge: info.1,
        was_charging: false
        }  
    )
    }

    pub fn update(&mut self) {
        let info = match crate::get_bat_info() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                std::process::exit(1)
            }
        };
        self.was_charging = self.charge;
        self.pct = info.0;
        self.charge = info.1


    }


}
