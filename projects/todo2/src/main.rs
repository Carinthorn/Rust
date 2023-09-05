use structopt::StructOpt;
mod cli;
mod task;

use cli::{Action::*, CommandLineArgs};
use anyhow::anyhow;
use task::Mission;
use std::path::PathBuf;


fn find_default_journal_file() -> Option<PathBuf>{
    home::home_dir().map(|mut path|{
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()>{
    // println!("{:#?}", cli::CommandLineArgs::from_args());
    let CommandLineArgs { 
        action, 
        journal_file,
    } = CommandLineArgs::from_args();

    //unpack file 
    let file_path = journal_file
        .or_else(find_default_journal_file) //handle case when journal file is not provided
        .ok_or(anyhow!("Failed to find journal file."))?; //handle case when journal file and default path resolution result in None

    match action{
        Add{ text } => task::add_task(file_path, Mission::new(text)),
        Complete{ number } => task::complete_task(file_path, number),
        Delete{ number } => task::delete_task(file_path, number),
        List => task::list_missions(file_path),
    }?;
    
    Ok(())
}
