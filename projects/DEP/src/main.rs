mod lib;
use lib::JSON_to_BSON;

fn main() {
    let data = 
    r#"{
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let result = JSON_to_BSON(data);
    println!("{:?}", result)
}




