use std::path::PathBuf; 
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action{
    /// Write tasks to the journal file. 
    Add{ 
        #[structopt()]
        text: String
    }, 
    /// Remove an entry from the journal file by position.
    Done{ 
        #[structopt()]
        position: usize
    }, 
    /// List all tasks in the journal file.
    List, 
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action: Action, 

    #[structopt(parse(from_os_str),short, long)]
    pub journal_file: Option<PathBuf>,
}

//print info = cargo run
//cargo run -- -j test-journal.json add "buy milk"]
//cargo run -- -j test-journal.json list
//cargo run -- -j test-journal.json done 2
