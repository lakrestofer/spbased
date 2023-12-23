use std::path::PathBuf;

use api::server;
use clap::Parser;

use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "Spbased")]
#[command(author = "la. Krestofer <lagrestofer@gmail.com>")]
// #[command(version = "1.0")]
#[command(about = "gRPC daemon for spbased ecosystem. See https://github.com/lakrestofer/spbased for more info", long_about = None)]
struct Cli {
    /// Override default config path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Default, Serialize, Deserialize)]
struct Config {
    server_config: server::ServerConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config_dir = config_dir().expect("could not retrieve user config directory. Is platform anything other than linux, macOS or windows?");
    // override the config location
    let config_dir = config_dir.join("spbased");
    let mut config_file = config_dir.join("config.toml");

    if let Some(path) = cli.config {
        if path.is_file() {
            config_file = path;
        }
    }

    // try to read the config file
    let config: Config = {
        match std::fs::read_to_string(&config_file) {
            // if it already exists, try to parse it
            Ok(str) => toml::from_str(&str)?,
            // if doesn't, write the default config to the file
            _ => {
                let config = Config::default();
                // create the dirs
                std::fs::create_dir_all(&config_dir)?;
                // write the default config to a file
                std::fs::write(&config_file, toml::ser::to_string(&config)?)?;
                config
            }
        }
    };

    // launch the server with the provided config
    server::run_server(config.server_config).await
}
