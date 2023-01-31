use clap::{arg, Parser, Subcommand};
use std::{path::PathBuf};


/// Represents an action that the program can perform.
#[derive(Debug, Subcommand)]
pub enum Action {
  /// Add a task to the journal file.
  #[command(long_flag = "add")]
  Add {
    /// Name of task to add.
    //#[arg(long)]
    task: String,
  },

  /// Remove an entry from the journal file by position.
  #[command(long_flag = "done")]
  Done {
    /// Number of the task to mark as done.
    //#[arg(long)]
    position: usize,
  },

  /// List all tasks in the journal file.
  #[command(long_flag = "list")]
  List,
}

/// Represents a parser and the set the arguments provided by the user through the CLI app.
#[derive(Debug, Parser)]
pub struct CommandLineArgs {
  /// Action to perform.
  #[command(subcommand)]
  pub action: Action,

  /// Represents the text file to save the TO-DO tasks.
  #[arg(short='j', long = "file", value_parser=clap::value_parser!(std::path::PathBuf))]
  pub journal_file: Option<PathBuf>,
}


