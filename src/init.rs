use crate::contact::Contacts;
use anyhow::Result;

pub fn init() -> Result<()> {
    Contacts::new().save_to_home()?;
    eprintln!("Storage successfully initialized.");
    Ok(())
}
