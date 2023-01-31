mod tasks;
use std::path::PathBuf;

use tasks::Task;
mod cli;
use clap::Parser;
use cli::CommandLineArgs;

/// Gets a default path for the journal file.
fn get_default_journal_file() -> Option<PathBuf> {
  // Append a JSON file name to the path of the HOME directory if that directory exists
  // Note: the map method works only with Some(value)
  home::home_dir().map(|mut path| {
    path.push(".rusty-journal.json");
    path
  })
}
fn main() -> anyhow::Result<()> {
  // Get the user arguments
  let cli: CommandLineArgs = CommandLineArgs::parse();

  // Check if the given file name
  // Note: the .or_else method works only with a None value.
  let journal_path = cli
    .journal_file
    .or_else(get_default_journal_file)
    .ok_or(anyhow::anyhow!("Failed to find the journal file!"))?;

  match cli.action {
    cli::Action::Add { task } => tasks::add_task(journal_path, Task::new(task)),
    cli::Action::Done { position } => tasks::complete_task(journal_path, position),
    cli::Action::List => tasks::list_tasks(journal_path),
  }?;

  Ok(())
}
