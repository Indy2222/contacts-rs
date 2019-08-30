use crate::contact::{Contact, Contacts};
use crate::error::InputError;
use dialoguer::Input;
use std::collections::HashMap;
use std::error::Error;

pub fn add_contact() -> Result<(), Box<dyn Error>> {
    let full_name = prompt("Full name")?;
    let mut entity_name = None;

    if full_name.is_none() {
        entity_name = prompt("Entity name")?;
    }

    if full_name.is_none() && entity_name.is_none() {
        return input_error!("One of full name or entity name must be given.");
    }

    if full_name.is_some() && entity_name.is_some() {
        return input_error!("Contact can't have both full name and entity name.");
    }

    let mut contact = if full_name.is_some() {
        Contact::with_full_name(full_name.unwrap())
    } else {
        Contact::with_entity_name(entity_name.unwrap())
    };

    let emails = prompt_map("E-mail address")?;
    contact.set_emails(emails);

    let tels = prompt_map("Telephone number")?;
    contact.set_tels(tels);

    let labels = prompt_map("Label")?;
    contact.set_labels(labels);

    let mut contacts = Contacts::load_from_home()?;
    contacts.add(contact);
    contacts.save_to_home()?;

    eprintln!("Contact successfully added.");

    Ok(())
}

fn prompt_map(name: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
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
            return Err(Box::new(InputError::new(format!(
                "The map already contains key {}",
                key
            ))));
        }

        map.insert(key, value);
    }

    Ok(map)
}

fn prompt(name: &str) -> Result<Option<String>, Box<dyn Error>> {
    let result = Input::<String>::new()
        .with_prompt(name)
        .allow_empty(true)
        .interact()?;

    if result.is_empty() {
        return Ok(None);
    }

    return Ok(Some(result));
}
