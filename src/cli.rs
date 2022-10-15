use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the OG-TODO file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the OG-TODO file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the OG-TODO file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "OG-TODO", about = "A command line to-do app powered by Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub og_todo_file: Option<PathBuf>,
}
