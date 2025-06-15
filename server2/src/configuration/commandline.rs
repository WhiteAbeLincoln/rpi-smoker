use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(name = "Aero", about = "Campbell Aero Meterological Application")]
pub struct CommandlineOpts {
    // FIX: change the default when doing a laws build
    /// Opens the specified yaml configuration file (can be without extension)
    #[arg(short, long, default_value = "Config.yaml", value_hint = clap::ValueHint::FilePath)]
    pub cfg: std::path::PathBuf,

    // Logging options
    /// Controls the log level
    #[arg(long, default_value = "error")]
    pub log_level: log::Level,
}
