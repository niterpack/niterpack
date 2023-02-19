#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub mods: Vec<Mod>
}

#[derive(Debug)]
pub struct Mod {
    pub file: String,
    pub download: String
}
