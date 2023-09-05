use structopt::StructOpt;
mod cli;
mod task;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use task::Task;
use std::path::PathBuf;


fn find_default_journal_file() -> Option<PathBuf>{
    home::home_dir().map(|mut path|{
        path.push(".rusty-journal.json");
        path
    })
}
fn main() -> anyhow::Result<()> {
    // cli::CommandLineArgs::from_args();
    // println!("{:#?}", cli::CommandLineArgs::from_args());
    let CommandLineArgs { 
        action, 
        journal_file
    } = CommandLineArgs::from_args();

    //unpack jourmal file 
    let journal_file = journal_file
    .or_else(find_default_journal_file)
    .ok_or(anyhow!("Failed to find journal file."))?;

    //perform action
    match action{
        Add{ text } => task::add_task(journal_file, Task::new(text)), 
        List => task::list_tasks(journal_file), 
        Done{ position } => task::complete_task(journal_file, position), 
    }?;

    Ok(())
}

//unpack jourmal file

