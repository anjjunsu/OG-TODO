use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

pub fn add_task(og_todo_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(og_todo_path)?;

    // Read file contents as a vector of tasks.
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind file
    file.seek(SeekFrom::Start(0))?;

    // Write updated task list
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}
