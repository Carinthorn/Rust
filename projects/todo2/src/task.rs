use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc, Local};
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::fmt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)] //apply trait to struct
pub struct Mission{
    pub mission_name: String, 

    #[serde(with = "ts_seconds")] //ts_second = module
    pub created_at: DateTime<Utc>,
    pub status: String
}

impl Mission{
    pub fn new(name: String) -> Mission{
        let created_at: DateTime<Utc> = Utc::now();
        Mission{
            mission_name: name, 
            created_at, 
            status: String::from("In progress")
        }
    }
}

impl fmt::Display for Mission{ //format display for custom type 
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} {} [{}]", self.mission_name, self.status, created_at) //write data to file without displaying on screen
    }
}

pub fn add_task(file_path: PathBuf, mission: Mission) -> Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut all_missions: Vec<Mission> = get_tasks(&file)?; //? = unwrap result type and propogate any further error that can occur 
    all_missions.push(mission);

    serde_json::to_writer(file, &all_missions)?;
    Ok(())
}

pub fn complete_task(file_path: PathBuf, number: usize) -> Result<()>{
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)?;
    let mut missions = get_tasks(&file)?;
    for i in 0..missions.len(){
        if i == number - 1{
            missions[i].status = String::from("Completed");
            break;
        }
    }
    file.set_len(0)?;
    serde_json::to_writer(file, &missions)?;
    Ok(())
}

pub fn delete_task(file_path: PathBuf, number: usize) -> Result<()>{
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)?; //? = unwrap result type and propogate any further error that can occur

    let mut missions = get_tasks(&file)?;
    if number > missions.len() || number == 0{
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid mission number"))
        //create new error type with error kind = invalid input with error msg - invalid mission number
        //errorKind = enum that represents type of error in rust , invalid inpu = error type
    }
    missions.remove(number -1);

    //write missions back 
    file.set_len(0)?; //clearing content of file
    serde_json::to_writer(file, &missions)?;
    Ok(())
}

pub fn list_missions(file_path: PathBuf) -> Result<()>{
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)?;
    let missions: Vec<Mission> = get_tasks(&file)?;
    if missions.is_empty(){
        println!("No tasks found");
    }else{
        for (i, mission) in missions.iter().enumerate(){
            println!("{} {} {} {}", i+1, mission.mission_name, mission.status, mission.created_at)
        }
    }
    Ok(())
}

//deserialized = convert from json to rust 
//serialized = convert from rust to json 
pub fn get_tasks(mut file_name: &File) -> Result<Vec<Mission>>{ //&File = reference to file object not actual one, FIle =represent file object on the file system
    file_name.seek(SeekFrom::Start(0))?;
    let missions = match serde_json::from_reader(file_name){ //return deserialized result type
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(), 
        Err(e) => Err(e)?
    };//missions = result type 
    file_name.seek(SeekFrom::Start(0))?;
    Ok(missions)
}
    