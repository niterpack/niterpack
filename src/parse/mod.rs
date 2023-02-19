use std::fs;
use std::path::{PathBuf};
use serde::{Deserialize};
use crate::modpack::{Mod, Modpack};

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

pub fn parse(path: PathBuf) -> Result<Modpack, String> {
    if !path.exists() || !path.is_dir() {
        return Err("not a directory".to_string());
    }

    let main_file = parse_main_file(path.clone())?;

    return Ok(Modpack {
        name: main_file.name,
        version: main_file.version,
        mods: parse_mods(path)?
    })
}

fn parse_main_file(mut path: PathBuf) -> Result<MainFile, String> {
    path.push("niter.json");

    if !path.exists() || !path.is_file() {
        return Err("'niter.json' not found".to_string());
    }

    serde_json::from_str(
        fs::read_to_string(path)
            .map_err(|err| err.to_string())?
            .as_str())
        .map_err(|err| err.to_string())
}

fn parse_mods(mut path: PathBuf) -> Result<Vec<Mod>, String> {
    path.push("mods");

    if !path.exists() || !path.is_dir() {
        return Ok(vec![]);
    }

    let mut result = Vec::new();

    for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
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

fn parse_mod_file(path: PathBuf) -> Result<ModFile, String> {
    serde_json::from_str(
        fs::read_to_string(path)
            .map_err(|err| err.to_string())?
            .as_str())
        .map_err(|err| err.to_string())
}
