use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub enum Action{
    ///Add a new todo item in String
    Add{
        #[structopt()]
        text: String
    }, 
    ///Enter the position of completed to do item 
    Complete{
        #[structopt()]
        number: usize
    }, 
    ///Delete the to do item according to the position
    Delete{
        #[structopt()]
        number: usize
    }, 
    ///List out list of todo items
    List
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Thun todo app",
    about = "A command line todo app written in Rust"
)]

pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action: Action, 

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>
}

