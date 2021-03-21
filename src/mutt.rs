use crate::actions::MatchAction;
use crate::contact::Contact;
use anyhow::{bail, Result};

pub struct Mutt {}

impl Mutt {
    pub fn new() -> Self {
        Mutt {}
    }
}

impl MatchAction for Mutt {
    fn process(&self, contacts: Vec<&mut Contact>) -> Result<bool> {
        if contacts.is_empty() {
            bail!("No contact has been matched.");
        }

        println!("{} contacts found.", contacts.len());

        for contact in contacts {
            for (email_name, email_address) in contact.emails.iter() {
                println!(
                    "{}\t{}\t({})",
                    email_address,
                    contact.full_name().or(contact.entity_name()).unwrap(),
                    email_name
                );
            }
        }

        Ok(false)
    }
}
