use csv::{ReaderBuilder, Writer, Trim};
use std::{io::Read, fs::File, error::Error}; 

//q1
struct Point{
    point : (f32,f32)
}
struct PolarPoint{
    polar : (f32,f32)
}

pub fn to_polar(pt_list: Vec<(f32, f32)>) -> Vec<(f32, f32)>{
    let mut polar_list = Vec::new();
    for pt in pt_list{
        let r: f32 = ((pt.0).powf(2.0) + (pt.1).powf(2.0)).sqrt();
        let t: f32 = (pt.1/pt.0).atan();
        polar_list.push((r,t));
    }
    polar_list
}


pub fn to_cartesian(pt_list: Vec<(f32, f32)>) -> Vec<(f32, f32)>{
    let mut cartesian_list = Vec::new();
    for pt in pt_list{
        let x: f32 = pt.0 * (pt.1).cos();
        let y: f32 = pt.0 * (pt.1).sin();
        cartesian_list.push((x,y));
    }
    cartesian_list
}


//q2
pub fn load_data(filename: &str) ->  Result<Vec<(f32, f32)>, Box<dyn Error>>{
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(file); 

    let mut output = Vec::new();
    for record in reader.records(){
        if let Ok(rec) = record {
            let v1 = rec[0].parse()?;
            let v2 = rec[1].parse()?;
            output.push((v1, v2));
        }
    }
    Ok(output)
}


pub fn save_data(filename: &str, file_type: &str , data: Vec<(f32, f32)>) -> Result<(), Box<dyn Error>>{
    let file = File::create(filename.to_owned() + file_type)?;

    if file_type == ".html"{
        let html_data = convert_to_html(data);
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_writer(file);

        writer.write_record(&[html_data])?;
        writer.flush()?;
        Ok(())
    }
    else if file_type == ".csv"{
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_writer(file);

        for i in data{
            writer.write_record(&[i.0.to_string(), i.1.to_string()])?;
        }
        writer.flush()?;
        Ok(())
    }
    else{
        println!("Invalid file type");
        Ok(())
    }
    
}

pub fn convert_to_html(data: Vec<(f32, f32)>) -> String {
    let mut html = String::new();
    html.push_str(&format!("{}",
r#"<style>
table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
}
</style>
"#
    ));
    html.push_str("<table>\n");
    html.push_str(&format!("{:2}<tr>\n", ""));
    html.push_str(&format!("{:4}<td>Points</td>\n", ""));
    html.push_str(&format!("{:2}</tr>\n", ""));

    for i in data{
        html.push_str(&format!("{:2}<tr>\n", ""));
        html.push_str(&format!("{:4}<td>{:?}</td>\n", "", i));
        html.push_str(&format!("{:2}<tr>\n", ""));
    }
    html.push_str("<table>\n");

    html
}

pub fn read_csv_cartesian(filename: &str) -> Vec<(f32, f32)>{
    let data = load_data(filename).unwrap();
    let polar_data = to_polar(data);

    polar_data
    //change to html
    // match save_data("output_polar", ".html", polar_data) {
    //     Ok(()) => println!("Successfully saved"),
    //     Err(_) => println!("Error"),
    // }
}

pub fn read_csv_polar(filename: &str) -> Vec<(f32, f32)>{
    let data = load_data(filename).unwrap();
    let cartesian_data = to_cartesian(data);
    cartesian_data
}


