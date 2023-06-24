use crate::{Manifest, Project};
use eyre::ContextCompat;
use log::info;
use owo_colors::{OwoColorize, Stream, Style};
use std::env;

#[derive(clap::Args)]
pub struct InitArgs;

impl InitArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let current_dir = env::current_dir().unwrap();

        let project = Project::from(Manifest::new(
            current_dir
                .file_name()
                .and_then(|name| name.to_os_string().into_string().ok())
                .wrap_err("failed to get name of the current directory")?,
            String::from("0.1.0"),
            None,
            None,
        ));

        project.write(current_dir)?;

        info!(
            "{} new modpack `{}`",
            "Created".if_supports_color(Stream::Stdout, |text| text
                .style(Style::new().green().bold())),
            &project.manifest.name
        );
        Ok(())
    }
}
