//Data serialization : structured data -> JSON/BSON/MessagePack/Base64
//csv, excel , xml 
//Data Deserialization : JSON/BSON/MessagePack/Base64 -> structured data
use bson::{to_document, Document, Bson, to_bson};
use serde_json::{self, json, Value};
use base64::{encode, decode};
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main(){

}

pub fn load_data(filename:&str) -> Result<String, Box<dyn Error>>{
    let mut file = File::open(filename).expect("File to open file");
    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(_) => {
            println!("Succeeded to read file {}", filename);
            return Ok(contents);
        }
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            return Err(Box::new(err));
        }
    }
}

// pub fn save_data(){
//JSON convertion 
pub fn JSON_to_BSON(data: &str) -> Bson{
    let json_data: serde_json::Value = serde_json::from_str(&data).expect("Failed to parse JSON"); 
    let bson_data = to_bson(&json_data).expect("failed to convert JSON to BSON");
    println!("BSON: {:?}", bson_data);
    return bson_data;
}

pub fn JSON_to_MessagePack(data: &str) -> {
    let json_data: serde_json::Value = serde_json::from_str(&data).expect("Failed to parse JSON");
    let msgpack_data = 
}



//question 
// ok to use preexisting crate like bson or csv or i have to implement it myself?'
// ways to implement like matching bson -> json  
// how to implement base64 encoding/decoding like text file -> binary file -> text file
//or do i have to write my own implementation of Serialize and Deserialize