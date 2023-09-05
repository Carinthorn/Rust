use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc, Local};
use serde::Deserialize; 
use serde::Serialize; 
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task{
    pub fn new(text: String) -> Task{
        let created_at: DateTime<Utc> = Utc::now();
        Task{ text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()>{
    //open file 
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    //consumer file contect as a vector of tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;
    tasks.push(task);

    //write  the modified task list back to file 
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}


pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()>{
    //Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    //Consume files content as vector of taks
    let mut tasks = collect_tasks(&file)?;

    //Remove the task
    if task_position == 0 || task_position > tasks.len(){
        return Err(Error::new(ErrorKind::InvalidInput,  "invalid task ID"));
    }
    tasks.remove(task_position - 1);

    //write the modieied task list back to the fil e
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn collect_tasks(mut file: &File) -> Result<Vec<Task>>{
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file){
        Ok(tasks) => tasks, 
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?, 
    };
    file.seek(SeekFrom::Start(0))?; 
    Ok(tasks)
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()>{
    //Open file
    let file = OpenOptions::new().read(true).open(journal_path)?;
    //parse file and collect tasks
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty(){
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks{
            println!("{}: {}", order, task);
            order += 1;
        }
    }
    Ok(()) 
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

