use crate::actions::MatchAction;
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

pub fn search(options: SearchOptions, action: Box<dyn MatchAction>) -> Result<()> {
    let mut contacts = Contacts::load_from_home()?;

    let results: Vec<&mut Contact> = contacts
        .contacts_mut()
        .iter_mut()
        .filter(|contact| is_match(contact, &options))
        .collect();

    let save = action.process(results)?;
    if save {
        contacts.save_to_home()?;
    }

    Ok(())
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
