use error::Result;
use structopt::StructOpt;

mod api;
mod color;
mod error;
mod help;
mod log;
mod opts;
mod service;
mod show;
mod ui;
mod yaml_path;

#[tokio::main]
async fn main() {
    match run(opts::Opts::from_args()).await {
        Ok(_) => (),
        Err(err) => eprintln!("{:?}", err),
    }
}

async fn run(opts: opts::Opts) -> Result<()> {
    opts.validate()?;
    opts.set_profile();

    match &opts.sub_command {
        Some(sub_command) => service::execute_command(sub_command, opts.clone()).await?,
        None => {
            ui::tui(opts, None, None).await?;
            ()
        }
    }
    Ok(())
}
