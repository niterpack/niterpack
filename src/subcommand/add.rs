use std::env;
use log::info;
use url::Url;
use crate::error::Result;
use crate::format::ProjectFormatter;
use crate::modrinth;
use crate::project::Mod;
use crate::project::source::Source;

#[derive(clap::Args)]
pub struct AddArgs {
    /// Source of the new mod, either url or modrinth version id
    source: String,

    /// Name of the new mod
    #[arg(short, long)]
    name: String
}

fn parse_source(s: &str) -> Result<Source> {
    if let Ok(_) = modrinth::get_version(s) {
        Ok(Source::Modrinth {
            version_id: s.into()
        })
    } else {
        Url::parse(s)?;
        Ok(Source::Download {
            url: s.into(),
        })
    }
}

impl AddArgs {
    pub fn run(&self) -> Result<()> {
        let mod_data = Mod::new(
            self.name.clone(),
            None,
            parse_source(&self.source)?
        );

        let formatter = ProjectFormatter::format(
            env::current_dir().unwrap()
        )?;

        formatter.create_mod(&mod_data)?;

        info!("Added mod `{}` to modpack", self.name);
        Ok(())
    }
}