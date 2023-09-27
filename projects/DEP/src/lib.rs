//Data serialization : structured data -> JSON/BSON/MessagePack/Base64
//csv, excel , xml 
//Data Deserialization : JSON/BSON/MessagePack/Base64 -> structured data
use bson::{to_document, Document, Bson, to_bson, decode};
use serde_json::{self, json, Value};
use base64::{encode, decode};
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main(){

}

//Main feature 
//Format Conversion : JSON -> BSON/MessagePack/Base64, BSON -> JSON/MessagePack/Base64, MessagePack -> JSON/BSON/Base64, Base64 -> JSON/BSON/MessagePack
//fn load data 
// pub fn save_data(){

// }


//JSON convertion 
pub fn JSON_to_BSON(data : &str) -> Bson{
    let json_data: serde_json::Value = serde_json::from_str(&data).expect("Failed to parse JSON"); 
    let bson_data = to_bson(&json_data).expect("failed to convert JSON to BSON");
    println!("BSON: {:?}", bson_data);
    bson_data
}

//problem
pub fn BSON_to_JSON(bson_data: Bson) -> serde_json::Value{
    let mut file = File::open("input.bson").expect("Failed to open");
    let mut bson_data = Vec::new();
    file.read_to_end(&mut bson_data).expect("Failed to read");

    let bson_doc = bson::decode(&bson_data).expect("Failed to decode");
    let json_value: Value = serde_json::from_str(&bson_doc.to_join()).expect("failed");

}

//deserilize Json -> Bson
//Base64 Encoding/Decoding -> Encode text/image/text-base protocol -> binary, decode binary to text
fn Base64_encode(text: &str) -> String{
    let encoded = encode(text);
    
}
