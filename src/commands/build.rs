use crate::ops;
use crate::Project;
use log::info;
use owo_colors::OwoColorize;
use owo_colors::Stream;
use owo_colors::Style;
use std::env;

#[derive(clap::Args)]
pub struct BuildArgs;

impl BuildArgs {
    pub fn run(&self) -> eyre::Result<()> {
        let current_dir = env::current_dir().unwrap();

        ops::build(&Project::read(&current_dir)?, current_dir.join("build"))?;

        info!(
            "{} modpack",
            "Built".if_supports_color(Stream::Stdout, |text| text
                .style(Style::new().green().bold()))
        );
        Ok(())
    }
}
