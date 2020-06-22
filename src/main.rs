use error::Result;
use opts::*;
use structopt::StructOpt;

mod cache;
mod color;
mod error;
mod opts;
mod service;
pub mod show;
mod ui;

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
            _ => service::execute_command(sub_command, opts.clone()).await?,
        },
        None => {
            ui::tui(opts, None, None).await?;
            ()
        }
    }
    Ok(())
}
