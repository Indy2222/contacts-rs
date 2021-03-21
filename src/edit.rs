use crate::actions::MatchAction;
use crate::contact::Contact;
use anyhow::{bail, Result};
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;

pub struct EditContact {}

impl EditContact {
    pub fn new() -> Self {
        EditContact {}
    }
}

impl MatchAction for EditContact {
    fn process(&self, mut contacts: Vec<&mut Contact>) -> Result<bool> {
        if contacts.is_empty() {
            bail!("No contact has been matched.");
        }
        if contacts.len() > 1 {
            bail!("More than one contacts matched.");
        }

        let contact = contacts.pop().unwrap();
        edit_contact(contact)?;
        Ok(true)
    }
}

enum Action {
    Finish,
    ChangeFullName,
    ChangeEntityName,
    AddEmail,
    RemoveEmail,
    ChangeEmail,
    AddTel,
    RemoveTel,
    ChangeTel,
    AddLabel,
    RemoveLabel,
    ChangeLabel,
}

fn edit_contact(contact: &mut Contact) -> Result<()> {
    loop {
        eprintln!("--------------------------------------------------");
        eprint!("{}", contact);
        eprintln!("--------------------------------------------------");

        let mut selections: Vec<String> = Vec::new();
        let mut actions: Vec<(Action, Option<String>)> = Vec::new();

        let mut add_action = |action: Action, key: Option<&str>| {
            let prompt = match action {
                Action::Finish => String::from("finish"),
                Action::ChangeFullName => String::from("change full name"),
                Action::ChangeEntityName => String::from("change entity name"),
                Action::AddEmail => String::from("add email"),
                Action::RemoveEmail => format!("remove email: {}", key.unwrap()),
                Action::ChangeEmail => format!("change email: {}", key.unwrap()),
                Action::AddTel => String::from("add tel"),
                Action::RemoveTel => format!("remove tel: {}", key.unwrap()),
                Action::ChangeTel => format!("change tel: {}", key.unwrap()),
                Action::AddLabel => String::from("add label"),
                Action::RemoveLabel => format!("remove label: {}", key.unwrap()),
                Action::ChangeLabel => format!("change label: {}", key.unwrap()),
            };

            selections.push(prompt);
            actions.push((action, key.map(String::from)));
        };

        add_action(Action::Finish, None);

        if contact.full_name().is_some() {
            add_action(Action::ChangeFullName, None);
        } else {
            add_action(Action::ChangeEntityName, None);
        }

        add_action(Action::AddEmail, None);
        for key in contact.emails.keys() {
            add_action(Action::RemoveEmail, Some(key));
            add_action(Action::ChangeEmail, Some(key));
        }

        add_action(Action::AddTel, None);
        for key in contact.tels.keys() {
            add_action(Action::RemoveTel, Some(key));
            add_action(Action::ChangeTel, Some(key));
        }

        add_action(Action::AddLabel, None);
        for key in contact.labels.keys() {
            add_action(Action::RemoveLabel, Some(key));
            add_action(Action::ChangeLabel, Some(key));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select action")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        let (action, key) = actions.get(selection).unwrap();
        let key = key.as_ref().map(String::from);

        match action {
            Action::Finish => break Ok(()),
            Action::ChangeFullName => {
                let full_name = Input::<String>::new()
                    .with_prompt("Full name")
                    .interact()
                    .unwrap();
                contact.set_full_name(full_name)?;
            }
            Action::ChangeEntityName => {
                let entity_name = Input::<String>::new()
                    .with_prompt("Entity name")
                    .interact()
                    .unwrap();
                contact.set_entity_name(entity_name)?;
            }
            Action::AddEmail => prompt_key_value("email", &mut contact.emails),
            Action::RemoveEmail => {
                contact.emails.remove(&key.unwrap());
            }
            Action::ChangeEmail => prompt_change_value("email", &mut contact.emails, key.unwrap()),
            Action::AddTel => prompt_key_value("telephone number", &mut contact.tels),
            Action::RemoveTel => {
                contact.tels.remove(&key.unwrap());
            }
            Action::ChangeTel => {
                prompt_change_value("telephone number", &mut contact.tels, key.unwrap())
            }
            Action::AddLabel => prompt_key_value("label", &mut contact.labels),
            Action::RemoveLabel => {
                contact.labels.remove(&key.unwrap());
            }
            Action::ChangeLabel => prompt_change_value("label", &mut contact.labels, key.unwrap()),
        }
    }
}

fn prompt_key_value(name: &str, map: &mut HashMap<String, String>) {
    let key = Input::<String>::new()
        .with_prompt(&format!("{} name", name))
        .interact()
        .unwrap();

    if map.contains_key(&key) {
        eprintln!("This key already exist!");
        return;
    }

    let value = Input::<String>::new().with_prompt(name).interact().unwrap();
    map.insert(key, value);
}

fn prompt_change_value(name: &str, map: &mut HashMap<String, String>, key: String) {
    let value = Input::<String>::new().with_prompt(name).interact().unwrap();
    map.insert(key, value);
}
