use crate::Project;
use eyre::{Result, WrapErr};
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

pub fn build(project: &Project, path: PathBuf) -> Result<()> {
    build_installation(project, path.join("installation"))
}

pub fn build_installation(project: &Project, path: PathBuf) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(&path).wrap_err("failed to remove installation file")?;
        } else {
            fs::remove_dir_all(&path).wrap_err("failed to remove installation directory")?;
        }
    }

    fs::create_dir_all(&path).wrap_err("failed to create installation directory")?;

    let mods_dir = path.join("mods");

    fs::create_dir(&mods_dir).wrap_err("failed to create mods directory inside installation")?;

    let client = reqwest::blocking::Client::builder()
        .build()
        .wrap_err("failed to create a reqwest client")?;

    for mod_data in &project.mods {
        let file_name = mod_data.file_name().wrap_err(format!(
            "failed to get file name of mod `{}`",
            mod_data.name
        ))?;

        let url = mod_data.download_url().wrap_err(format!(
            "failed to get download url of mod `{}`",
            mod_data.name
        ))?;

        info!("Downloading {}", file_name);

        download(&client, &mods_dir.join(&file_name), &url)
            .wrap_err(format!("failed to download mod `{}`", mod_data.name))?;
    }

    Ok(())
}

fn download(client: &reqwest::blocking::Client, path: &Path, url: &str) -> Result<()> {
    let response = client.get(url).send().wrap_err("failed to send request")?;

    let body = response.bytes()?;
    fs::write(path, &body).wrap_err(format!("failed to write to file `{:?}`", path))?;

    Ok(())
}
