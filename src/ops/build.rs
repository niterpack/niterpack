use crate::source::BuildSource;
use crate::Project;
use console::style;
use eyre::{Result, WrapErr};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use log::info;

pub fn build(project: &Project, path: PathBuf) -> Result<()> {
    info!("{} mods...", style("Fetching").cyan().bold());
    let sources = project.build_sources()?;

    build_instance(project, sources, path.join("instance"))
}

pub fn build_instance(project: &Project, sources: Vec<BuildSource>, path: PathBuf) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(&path).wrap_err("failed to remove instance file")?;
        } else {
            fs::remove_dir_all(&path).wrap_err("failed to remove instance directory")?;
        }
    }

    fs::create_dir_all(&path).wrap_err("failed to create instance directory")?;

    // Copy the configuration files
    if let Some(project_config) = &project.config_dir {
        let config_dir = path.join("config");

        copy_recursive(project_config, config_dir).wrap_err("failed to copy config files")?;
    }

    // Download all the mods
    let mods_dir = path.join("mods");

    fs::create_dir(&mods_dir).wrap_err("failed to create mods directory inside instance")?;

    let client = reqwest::blocking::Client::builder()
        .build()
        .wrap_err("failed to create a reqwest client")?;

    let progress = ProgressBar::new(sources.len() as u64);
    progress.set_style(
        ProgressStyle::with_template("  {prefix:.cyan.bold} mods [{bar:25}] {pos}/{len}")
            .unwrap()
            .progress_chars("= "),
    );
    progress.set_prefix("Downloading");

    for source in sources {
        progress.println(format!(
            "{} {}",
            style("Downloading").green().bold(),
            &source.file
        ));

        download(&client, &mods_dir.join(&source.file), &source.url)
            .wrap_err(format!("failed to download mod `{}`", &source.name))?;

        progress.inc(1)
    }

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
