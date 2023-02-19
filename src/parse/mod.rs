pub mod error;

use std::fs;
use std::path::{PathBuf};
use serde::{Deserialize};
use crate::modpack::{Mod, Modpack};
use crate::parse::error::{MainFileNotFound, NotADirectory, UnsupportedFormat};
use crate::Result;

#[derive(Deserialize)]
struct MainFile {
    format: String,
    name: String,
    version: String
}

#[derive(Deserialize)]
struct ModFile {
    download: String
}

pub fn parse(path: PathBuf) -> Result<Modpack> {
    if !path.exists() || !path.is_dir() {
        return Err(NotADirectory.into());
    }

    let main_file = parse_main_file(path.join("niter.json"))?;

    if main_file.format != "0beta" {
        return Err(UnsupportedFormat(main_file.format).into())
    }

    return Ok(Modpack {
        name: main_file.name,
        version: main_file.version,
        mods: parse_mods(path.join("mods"))?
    })
}

fn parse_main_file(mut path: PathBuf) -> Result<MainFile> {
    if !path.exists() || !path.is_file() {
        return Err(MainFileNotFound.into());
    }

    serde_json::from_str(fs::read_to_string(path)?.as_str())
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
