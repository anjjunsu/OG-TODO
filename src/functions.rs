use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

pub fn add_task(og_todo_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(og_todo_path)?;

    let mut tasks = collect_tasks(&file);

    // Write updated task list
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(og_todo_path: PathBuf, task_position: usize) -> Result<()> {
    // Task position check.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(og_todo_path)?;

    let mut tasks = collect_tasks(&file);

    // Truncate the file since removing task will produce smaller size file
    file.set_len(0);

    // Write the modified tasks to the file.
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

/// List to-do tasks
pub fn list_tasks(og_todo_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(og_todo_path)?;

    let tasks = collect_tasks(&file);

    if tasks.is_empty() {
        println!("Nothings to do!");
    } else {
        let mut order: usize = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

/// Read file to get tasks, and rewind file to the beginning
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;

    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind file
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
