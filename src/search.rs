use crate::contact::{Contact, Contacts};
use anyhow::Result;
use regex::Regex;

pub struct SearchOptions {
    full_name_regex: Option<Regex>,
    entity_name_regex: Option<Regex>,
}

impl SearchOptions {
    pub fn new() -> Self {
        SearchOptions {
            full_name_regex: None,
            entity_name_regex: None,
        }
    }

    pub fn set_full_name_regex(&mut self, full_name_regex: &str) -> Result<()> {
        self.full_name_regex = Some(Regex::new(full_name_regex)?);
        Ok(())
    }

    pub fn set_entity_name_regex(&mut self, entity_name_regex: &str) -> Result<()> {
        self.entity_name_regex = Some(Regex::new(entity_name_regex)?);
        Ok(())
    }
}

pub fn search_and_print(options: SearchOptions) -> Result<()> {
    let contacts = Contacts::load_from_home()?;
    let results = search(options, &contacts);

    if results.is_empty() {
        println!("No contacts found.");
        return Ok(());
    }

    println!("{} contacts found:\n", results.len());

    println!("--------------------------------------------------");
    for result in results {
        if let Some(full_name) = result.full_name() {
            println!("Full name: {}", full_name);
        }
        if let Some(entity_name) = result.entity_name() {
            println!("Entity name: {}", entity_name);
        }

        if !result.tels.is_empty() {
            println!("Telephone numbers:");
            for (key, value) in &result.tels {
                println!("  {}: {}", key, value);
            }
        }

        if !result.emails.is_empty() {
            println!("Emails:");
            for (key, value) in &result.emails {
                println!("  {}: {}", key, value);
            }
        }

        if !result.labels.is_empty() {
            println!("Labels:");
            for (key, value) in &result.labels {
                println!("  {}: {}", key, value);
            }
        }

        println!("--------------------------------------------------");
    }

    Ok(())
}

pub fn search(options: SearchOptions, contacts: &Contacts) -> Vec<&Contact> {
    contacts
        .contacts()
        .iter()
        .filter(|&contact| is_match(contact, &options))
        .collect()
}

fn is_match(contact: &Contact, options: &SearchOptions) -> bool {
    if let Some(full_name_regex) = &options.full_name_regex {
        match contact.full_name() {
            Some(full_name) => {
                if !full_name_regex.is_match(full_name) {
                    return false;
                }
            }
            None => {
                if options.entity_name_regex.is_none() {
                    return false;
                }
            }
        }
    }

    if let Some(entity_name_regex) = &options.entity_name_regex {
        match contact.entity_name() {
            Some(entity_name) => {
                if !entity_name_regex.is_match(entity_name) {
                    return false;
                }
            }
            None => {
                if options.full_name_regex.is_none() {
                    return false;
                }
            }
        }
    }

    return true;
}
