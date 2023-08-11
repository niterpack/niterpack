use crate::source::BuildSource;
use crate::Project;
use eyre::{Result, WrapErr};
use log::info;
use sha2::{Digest, Sha512};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

pub fn build(project: &Project, path: PathBuf) -> Result<()> {
    let sources = project.build_sources()?;
    build_instance(project, sources, path.join("instance"))
}

pub fn build_instance(
    project: &Project,
    mut sources: Vec<BuildSource>,
    path: PathBuf,
) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(&path).wrap_err("failed to create instance directory")?;
    }

    // Copy the configuration files
    if let Some(project_config) = &project.config_dir {
        let config_dir = path.join("config");

        if config_dir.exists() {
            fs::remove_dir_all(&config_dir).wrap_err("failed to remove config directory")?;
        }

        copy_recursive(project_config, config_dir).wrap_err("failed to copy config files")?;
    }

    // Download all the mods
    let mods_dir = path.join("mods");

    if !mods_dir.exists() {
        fs::create_dir(&mods_dir).wrap_err("failed to create mods directory inside instance")?;
    }

    let client = reqwest::blocking::Client::builder()
        .build()
        .wrap_err("failed to create a reqwest client")?;

    for mod_entry in fs::read_dir(&mods_dir)? {
        let mod_entry = mod_entry?;

        if let Some(index) = sources
            .iter()
            .position(|source| mod_entry.file_name() == OsString::from(&source.file))
        {
            let source = &sources[index];

            if let Some(source_hash) = &source.sha512 {
                let hash = sha512(&mod_entry.path()).wrap_err(format!(
                    "failed to generate sha512 for mod `{}`",
                    source.file
                ))?;

                if source_hash == &hash {
                    sources.remove(index);
                    continue;
                }
            }

            fs::remove_file(mod_entry.path())
                .wrap_err(format!("failed to remove mod `{}`", &source.file))?;

            download_source(&client, source, &mod_entry.path())
                .wrap_err(format!("failed to download mod `{}`", &source.file))?;

            sources.remove(index);
            continue;
        }

        fs::remove_file(mod_entry.path()).wrap_err(format!(
            "failed to remove mod `{}`",
            mod_entry.file_name().to_string_lossy()
        ))?;
    }

    for source in sources {
        download_source(&client, &source, &mods_dir.join(&source.file))
            .wrap_err(format!("failed to download mod `{}`", &source.file))?;
    }

    Ok(())
}

fn sha512(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut sha512 = Sha512::new();

    io::copy(&mut file, &mut sha512)?;

    Ok(hex::encode(sha512.finalize()))
}

fn download_source(
    client: &reqwest::blocking::Client,
    source: &BuildSource,
    path: &Path,
) -> Result<()> {
    info!("Downloading {}", &source.file);

    download(client, path, &source.url)?;

    Ok(())
}

fn download(client: &reqwest::blocking::Client, path: &Path, url: &str) -> Result<()> {
    let response = client.get(url).send().wrap_err("failed to send request")?;

    let body = response.bytes()?;
    fs::write(path, &body).wrap_err(format!("failed to write to file `{:?}`", path))?;

    Ok(())
}

fn copy_recursive<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    fs::create_dir(&to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            copy_recursive(path, to.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(path, to.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
