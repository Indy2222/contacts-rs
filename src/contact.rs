use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
    contacts: Vec<Contact>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    full_name: Option<String>,
    entity_name: Option<String>,
    pub tels: HashMap<String, String>,
    pub emails: HashMap<String, String>,
    pub labels: HashMap<String, String>,
}

impl Contacts {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    /// Load contacts data from an arbitrary path. The path must point to a
    /// valid JSON file with contact data.
    pub fn load_from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)
            .context("Could not load contacts. Make sure contacts are initialized.")?;
        let reader = BufReader::new(file);
        let contact = serde_json::from_reader(reader).context("Error while loading contacts.")?;
        Ok(contact)
    }

    /// Load contacts data from a standard path. See `load_from_path()`.
    pub fn load_from_home() -> Result<Self> {
        let data_path = Self::get_contacts_file(false)?;
        Self::load_from_path(data_path.as_path())
    }

    /// Save contact data to an arbitrary path. If the file already exists it
    /// will be truncated.
    pub fn save_to_path<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file = File::create(path).context("Error while saving contacts.")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self).context("Error while saving contacts.")?;
        Ok(())
    }

    /// Save contacts to standard path. Pre-existing file is truncated.
    pub fn save_to_home(&self) -> Result<()> {
        let data_path = Self::get_contacts_file(true)?;
        self.save_to_path(data_path)
    }

    pub fn contacts_mut(&mut self) -> &mut [Contact] {
        self.contacts.as_mut()
    }

    pub fn add(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    /// Get standard contacts JSON file path and create.
    ///
    /// # Arguments
    ///
    /// * `create_dir` - create standard contacts directory if it does not
    ///   exist.
    fn get_contacts_file(create_dir: bool) -> Result<PathBuf> {
        let mut data_path = Self::get_contacts_dir()?;

        if create_dir {
            fs::create_dir_all(data_path.as_path()).with_context(|| {
                format!(
                    "Error during creation of directory with contacts: {}",
                    data_path.display()
                )
            })?;
        }

        data_path.push("contacts.json");
        Ok(data_path)
    }

    /// Get path to standard contacts directory.
    fn get_contacts_dir() -> Result<PathBuf> {
        let mut data_path = match env::var("XDG_DATA_HOME") {
            Ok(val) => PathBuf::from(val),
            Err(_) => {
                let home = match env::var("HOME") {
                    Ok(home) => home,
                    Err(_) => bail!("Neither XDG_DATA_HOME nor HOME environment variable set."),
                };
                let mut path = PathBuf::from(home);
                path.push(".local");
                path.push("share");
                path
            }
        };

        data_path.push("conn-rs");
        Ok(data_path)
    }
}

impl Contact {
    pub fn with_full_name(full_name: String) -> Self {
        Self {
            full_name: Some(full_name),
            entity_name: None,
            tels: HashMap::new(),
            emails: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn with_entity_name(entity_name: String) -> Self {
        Self {
            full_name: None,
            entity_name: Some(entity_name),
            tels: HashMap::new(),
            emails: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn set_full_name(&mut self, full_name: String) -> Result<()> {
        if self.entity_name.is_some() {
            bail!("Full name and entity name cannot be set at the same time.");
        }
        self.full_name = Some(full_name);
        Ok(())
    }

    pub fn set_entity_name(&mut self, entity_name: String) -> Result<()> {
        if self.full_name.is_some() {
            bail!("Full name and entity name cannot be set at the same time.");
        }
        self.entity_name = Some(entity_name);
        Ok(())
    }

    pub fn full_name(&self) -> Option<&str> {
        self.full_name.as_ref().map(String::as_ref)
    }

    pub fn entity_name(&self) -> Option<&str> {
        self.entity_name.as_ref().map(String::as_ref)
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(full_name) = &self.full_name {
            writeln!(f, "Full name: {}", full_name)?;
        }
        if let Some(entity_name) = &self.entity_name {
            writeln!(f, "Entity name: {}", entity_name)?;
        }

        if !self.tels.is_empty() {
            writeln!(f, "Telephone numbers:")?;
            for (key, value) in &self.tels {
                writeln!(f, "  {}: {}", key, value)?;
            }
        }

        if !self.emails.is_empty() {
            writeln!(f, "Emails:")?;
            for (key, value) in &self.emails {
                writeln!(f, "  {}: {}", key, value)?;
            }
        }

        if !self.labels.is_empty() {
            writeln!(f, "Labels:")?;
            for (key, value) in &self.labels {
                writeln!(f, "  {}: {}", key, value)?;
            }
        }

        Ok(())
    }
}
