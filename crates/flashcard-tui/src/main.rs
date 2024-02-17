use flashcard_tui::preamble::*;
use flashcard_tui::{app::App, args::Args, config::Config, util::error_handling_setup};

#[tokio::main]
async fn main() -> Result<()> {
    // install error handling/reporting handlers
    error_handling_setup()?;

    // first we parse cli arguments
    let args = Args::parse();
    // read in the config from the
    let config = Config::read()?;

    let app = App::build(args, config)?;

    app.run()?;

    Ok(())
}
