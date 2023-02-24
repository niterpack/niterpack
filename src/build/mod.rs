use std::fs;
use std::path::PathBuf;
use crate::log;
use crate::error::Result;
use crate::project::Project;

pub fn build(project: Project, path: PathBuf) -> Result<()> {
    build_installation(project, path.join("installation"))
}

pub fn build_installation(project: Project, path: PathBuf) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(path.clone())?
        } else {
            fs::remove_dir_all(path.clone())?
        }
    }

    fs::create_dir_all(path.clone())?;

    let mods_dir = path.join("mods");
    fs::create_dir(mods_dir.clone())?;

    let client = reqwest::blocking::Client::builder()
        .build()?;

    for mod_data in project.mods {
        log!("Downloading {}", mod_data.file);

        let response = client.get(mod_data.download).send()?;
        let body = response.text()?;
        let mut file = fs::File::create(mods_dir.join(mod_data.file))?;

        std::io::copy(&mut body.as_bytes(), &mut file)?;
    }

    Ok(())
}
