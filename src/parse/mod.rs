pub mod error;

use std::fs;
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::parse::error::{FormatValueExpected, MainFileAlreadyExists, MainFileNotFound, ModAlreadyAdded, NotADirectory, UnsupportedFormat};
use crate::project::{Project, Mod};
use crate::Result;

const SUPPORTED_FORMAT: &str = "0beta";

#[derive(Serialize, Deserialize)]
struct MainFile {
    format: String,
    name: String,
    version: String
}

#[derive(Serialize, Deserialize)]
struct ModFile {
    download: String
}

pub fn create_mod_file(mod_data: &Mod, path: PathBuf) -> Result<()> {
    if path.exists() {
        return Err(ModAlreadyAdded(mod_data.file.clone()).into());
    }

    let mod_file = ModFile {
        download: mod_data.download.clone()
    };

    serde_json::to_writer_pretty(fs::File::create(path)?, &mod_file)
        .map_err(|err| err.into())
}

pub fn create_main_file(project: &Project, path: PathBuf) -> Result<()> {
    if path.exists() {
        return Err(MainFileAlreadyExists.into());
    }

    let main_file = MainFile {
        format: SUPPORTED_FORMAT.into(),
        name: project.name.clone(),
        version: project.version.clone()
    };

    serde_json::to_writer_pretty(fs::File::create(path)?, &main_file)
        .map_err(|err| err.into())
}

pub fn parse(path: PathBuf) -> Result<Project> {
    if !path.exists() || !path.is_dir() {
        return Err(NotADirectory.into());
    }

    let main_file = parse_main_file(path.join("niter.json"))?;

    return Ok(Project {
        name: main_file.name,
        version: main_file.version,
        mods: parse_mods(path.join("mods"))?
    })
}

fn parse_main_file(path: PathBuf) -> Result<MainFile> {
    if !path.exists() || !path.is_file() {
        return Err(MainFileNotFound.into());
    }

    let main_file: Value = serde_json::from_str(fs::read_to_string(path)?.as_str())?;
    let format = main_file["format"].as_str().ok_or(FormatValueExpected)?;

    if format != SUPPORTED_FORMAT {
        return Err(UnsupportedFormat(format.into()).into())
    }

    serde_json::from_value(main_file)
        .map_err(|err| err.into())
}

fn parse_mods(path: PathBuf) -> Result<Vec<Mod>> {
    if !path.exists() || !path.is_dir() {
        return Ok(vec![]);
    }

    let mut result = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() || path.extension() != Some("json".as_ref()) {
            continue;
        }

        let mod_file = parse_mod_file(path.clone())?;
        result.push(Mod {
            download: mod_file.download,
            file: path.with_extension("jar").file_name().unwrap().to_os_string().into_string().unwrap()
        })
    }

    Ok(result)
}

fn parse_mod_file(path: PathBuf) -> Result<ModFile> {
    serde_json::from_str(fs::read_to_string(path)?.as_str())
        .map_err(|err| err.into())
}
