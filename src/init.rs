use crate::contact::Contacts;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    Contacts::new().save_to_home()?;
    eprintln!("Storage successfully initialized.");
    Ok(())
}
