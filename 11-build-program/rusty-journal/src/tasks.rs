use std::{
  fs::{File, OpenOptions},
  io::{Seek, SeekFrom},
  path::PathBuf,
};

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a TO-DO task to be performed.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  /// Task description
  pub text: String,

  /// Timestamp of the task's creation.
  #[serde(with = "ts_seconds")]
  pub created_at: DateTime<Utc>,
}

impl Task {
  /// Creates a new instance of [`Task`].
  /// 
  /// # Arguments
  /// 
  /// * `text`: Task description.
  pub fn new(text: String) -> Self {
    Self {
      text: text,
      created_at: Utc::now(),
    }
  }
}


impl std::fmt::Display for Task {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let created_at = self.created_at.with_timezone(&chrono::Local).format("%F %H:%M");
    // Write with a lef-aligned string padded with 50 spaces
    write!(f, "{:<50} [{}]", self.text, created_at)
  }
}


/// Adds the given task to the list of tasks and saves the list to the given file path.
/// 
/// # Arguments
/// 
/// * `journal_path`: Path of the file that contains the task list.
/// * `task`: Task object to be added to the list of tasks.
pub fn add_task(
  journal_path: PathBuf,
  task: Task,
) -> std::io::Result<()> {
  // Open the file with the three modes
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(journal_path)?;

  // Load the file contents into a vector of tasks.
  let mut tasks: Vec<Task> = collect_tasks(&file)?;

  // Write the given task into the file
  tasks.push(task);
  serde_json::to_writer(file, &tasks)?;

  Ok(())
} // end fn add_task()


/// Takes the task indicated by the given position and removes it from the list of TO-DOs.
/// 
/// # Arguments
/// 
/// * `journal_path`: Path of the file that contains the task list.
/// * `task_position`: Position of the task to be removed as it was completed.
pub fn complete_task(
  journal_path: PathBuf,
  task_position: usize,
) -> std::io::Result<()> {
  // Open the file with the three modes
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(journal_path)?;

  // Load the file contents into a vector of tasks.
  let mut tasks: Vec<Task> = collect_tasks(&file)?;

  // Check if the position value is valid
  if task_position == 0 || task_position > tasks.len() {
    return Err(std::io::Error::new(
      std::io::ErrorKind::InvalidInput,
      format!("{} is not valid task number", task_position),
    ));
  }
  // Remove the task
  tasks.remove(task_position - 1);

  // Truncate the file contents, so we write in a blank file.
  file.set_len(0)?;
  // Write the tasks back into the file
  serde_json::to_writer(file, &tasks)?;

  Ok(())
} // end fn complete_task()

/// Creates a collection of tasks from the given file object.
///
/// # Arguments
///
/// * `file`: File object to load tasks.
///
fn collect_tasks(mut file: &File) -> std::io::Result<Vec<Task>> {
  // Go to the first position (rewind)
  file.seek(SeekFrom::Start(0))?;

  // Load the file contents into a vector of tasks.
  let tasks: Vec<Task> = match serde_json::from_reader(file) {
    // The file has a set of tasks.
    Ok(tasks) => tasks,
    // The file is empty.
    Err(error) if error.is_eof() => Vec::new(),
    // Any other IO error.
    Err(error) => Err(error)?,
  };

  // Return to the first position (rewind)
  file.seek(SeekFrom::Start(0))?;

  Ok(tasks)
} // end fn collect_tasks()

pub fn list_tasks(journal_path: PathBuf) -> std::io::Result<()> {
  // Open the file
  let file = OpenOptions::new().read(true).open(journal_path)?;

  // Load the tasks 
  let tasks: Vec<Task> = collect_tasks(&file)?;

  // List the tasks with a human numbering
  if tasks.is_empty() {
    println!("The tasks list is empty.");
  }
  else {
    for (counter, task) in tasks.into_iter().enumerate() {
      println!("{}: {}", counter + 1, task);
    }
  }

  Ok(())
}
