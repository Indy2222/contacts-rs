use crate::contact::Contacts;
use anyhow::{bail, Context, Result};
use std::ffi::OsStr;
use std::iter::IntoIterator;
use std::process::Command;

pub fn call<S, I>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let contacts_dir = Contacts::get_contacts_dir()?;
    if !contacts_dir.is_dir() {
        bail!("Directory with contacts does not yet exist. You have to initialize it first.");
    }

    let exit_status = Command::new("git")
        .current_dir(contacts_dir)
        .args(args)
        .spawn()
        .context("Failed to call git.")?
        .wait()
        .context("Failed to execute git command.")?;

    if !exit_status.success() {
        let exit_code = match exit_status.code() {
            Some(code) => format!("{}", code),
            None => "None".to_owned(),
        };
        bail!("git finished with non-success exit code: {}", exit_code);
    }

    Ok(())
}
