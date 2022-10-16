mod cli;
mod tasks;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn find_default_og_todo_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".og-todo.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get command-line args
    let CommandLineArgs {
        action,
        og_todo_file,
    } = CommandLineArgs::from_args();

    let og_todo_file = og_todo_file
        .or_else(find_default_og_todo_file)
        .ok_or(anyhow!("Failed to find og to-do file."))?;

    match action {
        Add { text } => tasks::add_task(og_todo_file, Task::new(text)),
        List => tasks::list_tasks(og_todo_file),
        Done { position } => tasks::complete_task(og_todo_file, position),
    }?;

    Ok(())
}
