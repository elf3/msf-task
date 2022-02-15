mod cli;
mod task;

use std::path::PathBuf;
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use task::Task;
use anyhow::anyhow;
fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
fn main() -> anyhow::Result<()>{
    let CommandLineArgs {
        action,
        file
    } = CommandLineArgs::from_args();
    let file = file.or_else(find_default_journal_file).ok_or(anyhow!("Failed to find journal file."))?;
    match action {
        Add {text} => task::add_task(file, Task::new(text)),
        List  => task::list_task(file),
        Done {position} => task::finish_task(file, position),
    }?;
    Ok(())
}
