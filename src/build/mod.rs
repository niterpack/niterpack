use std::fs;
use std::path::PathBuf;
use crate::modpack::Modpack;

pub fn build(modpack: Modpack, path: PathBuf) -> Result<(), String> {
    build_installation(modpack, path.join("installation"))
}

pub fn build_installation(modpack: Modpack, path: PathBuf) -> Result<(), String> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(path.clone()).map_err(|err| err.to_string())?
        } else {
            fs::remove_dir_all(path.clone()).map_err(|err| err.to_string())?
        }
    }

    fs::create_dir_all(path.clone()).map_err(|err| err.to_string())?;

    let mods_dir = path.join("mods");
    fs::create_dir(mods_dir.clone()).map_err(|err| err.to_string())?;

    let client = reqwest::blocking::Client::builder()
        .build()
        .map_err(|err| err.to_string())?;

    for mod_data in modpack.mods {
        println!("Downloading {}", mod_data.file);

        let response = client.get(mod_data.download)
            .send()
            .map_err(|err| err.to_string())?;

        let body = response.text()
            .map_err(|err| err.to_string())?;

        let mut file = fs::File::create(mods_dir.join(mod_data.file))
            .map_err(|err| err.to_string())?;

        std::io::copy(&mut body.as_bytes(), &mut file).map_err(|err| err.to_string())?;
    }

    Ok(())
}
