use crate::contact::Contact;
use anyhow::Result;

pub trait MatchAction {
    /// Process contacts and return true if the contacts have been modified
    /// along the way, in which case they will be saved.
    fn process(&self, contacts: Vec<&mut Contact>) -> Result<bool>;
}
