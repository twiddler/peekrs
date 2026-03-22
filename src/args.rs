use clap::Parser;
use std::path::PathBuf;

/// A file server with live reload
#[derive(Parser)]
#[command(name = "peekrs", version)]
pub struct Args {
    /// Directory to serve
    #[arg(default_value = ".", value_parser = parse_canonical_dir)]
    pub dir: PathBuf,

    /// Port to listen on
    #[arg(short, long, default_value = "3001")]
    pub port: u16,
}

fn parse_canonical_dir(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .canonicalize()
        .map_err(|_| format!("directory '{}' must exist", s))
}
