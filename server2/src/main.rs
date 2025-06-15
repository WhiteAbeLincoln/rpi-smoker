use anyhow::Context;
use clap::Parser;
use configuration::{commandline::CommandlineOpts, types::AeroCfg};
use flexi_logger::Logger;

mod configuration;
mod util;

fn mymain() -> anyhow::Result<()> {
    let opt = CommandlineOpts::parse();
    let _logger = Logger::try_with_env_or_str(opt.log_level.as_str())?
        .adaptive_format_for_stderr(flexi_logger::AdaptiveFormat::Detailed)
        .start()?;

    let cfg_file = std::fs::File::open(&opt.cfg)
        .with_context(|| format!("Failed to open config '{}'", opt.cfg.to_string_lossy()))?;
    let cfg_data: AeroCfg = serde_json::from_reader(cfg_file)
        .with_context(|| format!("Failed to parse config '{}'", opt.cfg.to_string_lossy()))?;
    let cfg_str = serde_json::to_string(&cfg_data)
        .with_context(|| format!("Failed to serialize config '{}'", opt.cfg.to_string_lossy()))?;

    log::info!("Got config data: {}", cfg_str);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // simply call mymain and use the logger to output the error message
    // if creating the logger caused the error, then the default main behavior will
    // log the message to stderr
    mymain().map_err(|e| {
        log::error!("{:?}", e);
        e
    })
}
