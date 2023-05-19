use crate::source::BuildSource;
use crate::Project;
use eyre::{Result, WrapErr};
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

pub fn build(project: &Project, path: PathBuf) -> Result<()> {
    let sources = project.build_sources()?;
    build_instance(sources, path.join("instance"))
}

pub fn build_instance(sources: Vec<BuildSource>, path: PathBuf) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(&path).wrap_err("failed to remove instance file")?;
        } else {
            fs::remove_dir_all(&path).wrap_err("failed to remove instance directory")?;
        }
    }

    fs::create_dir_all(&path).wrap_err("failed to create instance directory")?;

    let mods_dir = path.join("mods");

    fs::create_dir(&mods_dir).wrap_err("failed to create mods directory inside instance")?;

    let client = reqwest::blocking::Client::builder()
        .build()
        .wrap_err("failed to create a reqwest client")?;

    for source in sources {
        info!("Downloading {}", &source.file);

        download(&client, &mods_dir.join(&source.file), &source.url)
            .wrap_err(format!("failed to download mod `{}`", &source.name))?;
    }

    Ok(())
}

fn download(client: &reqwest::blocking::Client, path: &Path, url: &str) -> Result<()> {
    let response = client.get(url).send().wrap_err("failed to send request")?;

    let body = response.bytes()?;
    fs::write(path, &body).wrap_err(format!("failed to write to file `{:?}`", path))?;

    Ok(())
}
