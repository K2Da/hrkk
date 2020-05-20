use error::Result;
use opts::*;
use structopt::StructOpt;

mod cache;
mod color;
mod error;
mod info;
mod opts;
mod service;
pub mod show;
mod skimmer;

#[tokio::main]
async fn main() {
    match run(opts::Opts::from_args()).await {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

async fn run(opts: opts::Opts) -> Result<()> {
    opts.validate()?;
    opts.set_profile();

    match &opts.sub_command {
        Some(sub_command) => match sub_command {
            SubCommand::Cache { command } => match command {
                CacheCommand::List => cache::list()?,
                CacheCommand::Clear => cache::clear()?,
            },
            _ => service::execute_command(sub_command, &opts).await?,
        },
        None => {
            let selected_menu = crate::skimmer::commands::skim(service::all_resources(), &opts)?;
            for menu in selected_menu {
                for resource in service::all_resources() {
                    if resource.name() == menu.output() {
                        service::execute(&*resource, &opts).await?
                    }
                }
            }
            ()
        }
    }
    Ok(())
}
