use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
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
    tels: HashMap<String, String>,
    emails: HashMap<String, String>,
    labels: HashMap<String, String>,
}

impl Contacts {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    /// Load contacts data from an arbitrary path. The path must point to a
    /// valid JSON file with contact data.
    pub fn load_from_path<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let contact = serde_json::from_reader(reader)?;
        Ok(contact)
    }

    /// Load contacts data from a standard path. See `load_from_path()`.
    pub fn load_from_home() -> Result<Self, Box<dyn Error>> {
        let data_path = Self::get_contacts_file(false)?;
        Self::load_from_path(data_path.as_path())
    }

    /// Save contact data to an arbitrary path. If the file already exists it
    /// will be truncated.
    pub fn save_to_path<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }

    /// Save contacts to standard path. Pre-existing file is truncated.
    pub fn save_to_home(&self) -> Result<(), Box<dyn Error>> {
        let data_path = Self::get_contacts_file(true)?;
        self.save_to_path(data_path)
    }

    pub fn contacts(&self) -> &[Contact] {
        self.contacts.as_ref()
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
    fn get_contacts_file(create_dir: bool) -> Result<PathBuf, Box<dyn Error>> {
        let mut data_path = Self::get_contacts_dir()?;

        if create_dir {
            fs::create_dir_all(data_path.as_path())?;
        }

        data_path.push("contacts.json");
        Ok(data_path)
    }

    /// Get path to standard contacts directory.
    fn get_contacts_dir() -> Result<PathBuf, Box<dyn Error>> {
        let mut data_path = match env::var("XDG_DATA_HOME") {
            Ok(val) => PathBuf::from(val),
            Err(_) => {
                let home = env::var("HOME")?;
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

    pub fn set_tels(&mut self, tels: HashMap<String, String>) {
        self.tels = tels;
    }

    pub fn set_emails(&mut self, emails: HashMap<String, String>) {
        self.emails = emails;
    }

    pub fn set_labels(&mut self, labels: HashMap<String, String>) {
        self.labels = labels;
    }

    pub fn full_name(&self) -> Option<&str> {
        self.full_name.as_ref().map(String::as_ref)
    }

    pub fn entity_name(&self) -> Option<&str> {
        self.entity_name.as_ref().map(String::as_ref)
    }

    pub fn tels(&self) -> &HashMap<String, String> {
        &self.tels
    }

    pub fn emails(&self) -> &HashMap<String, String> {
        &self.emails
    }

    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
