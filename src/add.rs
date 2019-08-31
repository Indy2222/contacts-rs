use crate::contact::{Contact, Contacts};
use anyhow::{bail, Result};
use dialoguer::Input;
use std::collections::HashMap;

pub fn add_contact() -> Result<()> {
    let (entity_name, full_name) = loop {
        let full_name = prompt("Full name")?;

        let entity_name = if full_name.is_none() {
            prompt("Entity name")?
        } else {
            None
        };

        if full_name.is_none() && entity_name.is_none() {
            eprintln!("One of full name or entity name must be given.");
            continue;
        }

        break (entity_name, full_name);
    };

    let mut contact = if full_name.is_some() {
        Contact::with_full_name(full_name.unwrap())
    } else {
        Contact::with_entity_name(entity_name.unwrap())
    };

    contact.emails = prompt_map("E-mail address")?;
    contact.tels = prompt_map("Telephone number")?;
    contact.labels = prompt_map("Label")?;

    let mut contacts = Contacts::load_from_home()?;
    contacts.add(contact);
    contacts.save_to_home()?;

    eprintln!("Contact successfully added.");

    Ok(())
}

fn prompt_map(name: &str) -> Result<HashMap<String, String>> {
    eprintln!(
        "\nYou will be repeatedly asked for {} until you enter empty value.\n",
        name,
    );

    let mut map: HashMap<String, String> = HashMap::new();

    loop {
        let key = match prompt(&format!("{} name", name))? {
            Some(key) => key,
            None => break,
        };

        let value = prompt(&format!("{}", name))?;
        if value.is_none() {
            break;
        }
        let value = value.unwrap();

        if map.contains_key(&key) {
            bail!("The map already contains key {}", key);
        }

        map.insert(key, value);
    }

    Ok(map)
}

fn prompt(name: &str) -> Result<Option<String>> {
    let result = Input::<String>::new()
        .with_prompt(name)
        .allow_empty(true)
        .interact()?;

    if result.is_empty() {
        return Ok(None);
    }

    return Ok(Some(result));
}
