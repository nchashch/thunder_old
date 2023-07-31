use clap::Parser as _;

mod app;
mod cli;
mod thunder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let config = cli.get_config()?;
    let _app = app::App::new(&config.datadir, config.net_addr, config.main_addr)?;
    Ok(())
}
