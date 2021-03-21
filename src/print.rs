use crate::actions::MatchAction;
use crate::contact::Contact;
use anyhow::{bail, Result};

pub struct PrintExporter {}

impl PrintExporter {
    pub fn new() -> Self {
        PrintExporter {}
    }
}

impl MatchAction for PrintExporter {
    fn process(&self, contacts: Vec<&mut Contact>) -> Result<bool> {
        match contacts.len() {
            0 => bail!("No contacts found."),
            1 => println!("One contact found:"),
            n => println!("{} contacts found:", n),
        }

        println!("--------------------------------------------------");
        for result in contacts {
            print!("{}", result);
            println!("--------------------------------------------------");
        }

        Ok(false)
    }
}
