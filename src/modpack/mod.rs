
#[derive(Debug)]
pub struct Modpack {
    pub name: String,
    pub version: String,
    pub mods: Vec<Mod>
}

#[derive(Debug)]
pub struct Mod {
    pub file: String,
    pub download: String
}