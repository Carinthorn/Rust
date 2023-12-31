use csv::{ReaderBuilder, Trim};
use std::{fs::File, error::Error}; 

pub trait DataItem {
    fn get_point(&self) -> (f32, f32);
}

//q1
#[derive(Debug)]
pub struct Point{
    pub point : (f32,f32)
}

#[derive(Debug)]
pub struct PolarPoint{
    pub point : (f32,f32)
}

impl DataItem for Point {
    fn get_point(&self) -> (f32, f32) {
        self.point
    }
}

impl DataItem for PolarPoint {
    fn get_point(&self) -> (f32, f32) {
        self.point
    }
}

pub fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint>{
    let mut polar_list: Vec<PolarPoint> = Vec::new();
    for pt in pt_list{
        let r: f32 = ((pt.point.0).powf(2.0) + (pt.point.1).powf(2.0)).sqrt();
        let t: f32 = (pt.point.1/pt.point.0).atan();
        polar_list.push(PolarPoint {  point : (r,t) });
    }
    polar_list
}

pub fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point>{
    let mut cartesian_list: Vec<Point> = Vec::new();
    for pt in pt_list{
        let x: f32 = pt.point.0 * (pt.point.1).cos();
        let y: f32 = pt.point.0 * (pt.point.1).sin();
        cartesian_list.push( Point { point: (x,y)});
    }
    cartesian_list
}

pub fn load_data(filename: &str) ->  Result<Vec<(f32, f32)>, Box<dyn Error>>{
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(file); 

    let mut output: Vec<(f32, f32)>= Vec::new();
    for record in reader.records(){
        if let Ok(rec) = record {
            let v1 = rec[0].parse()?;
            let v2 = rec[1].parse()?;
            output.push((v1, v2));
        }
    }
    Ok(output)
}

pub fn save_data<T:DataItem>(filename: &str, file_type: &str , data: Vec<T>) -> Result<(), Box<dyn Error>>{
    let file = File::create(filename.to_owned() + file_type)?;
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_writer(file);

    match file_type {
        ".html" => {
            let html_data = convert_to_html(data);
            writer.write_record(&[html_data])?;
        }
        ".csv" => {
            for i in &data{
                let (x, y) = i.get_point();
                writer.write_record(&[x.to_string(), y.to_string()])?;
            }
        }
        _ => {
            println!("Invalid file type");
            return Ok(());
        }
    }
    writer.flush()?;
    Ok(())
}

pub fn convert_to_html<T:DataItem>(data: Vec<T>) -> String {
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
        let (x, y) = i.get_point();
        html.push_str(&format!("{:2}<tr>\n", ""));
        html.push_str(&format!("{:4}<td>({},{})</td>\n", "", x, y));
        html.push_str(&format!("{:2}<tr>\n", ""));
    }
    html.push_str("<table>\n");
    html
}

pub fn read_csv_cartesian(filename: &str) -> Vec<PolarPoint>{
    let data = load_data(filename).unwrap();
    let mut car_data: Vec<Point> = Vec::new();
    for i in data{
        let p1 = Point { point: (i.0, i.1) };
        car_data.push(p1);
    }
    let polar_data = to_polar(car_data); 
    polar_data
}

pub fn read_csv_polar(filename: &str) -> Vec<Point>{
    let data = load_data(filename).unwrap();
    let mut polar_data: Vec<PolarPoint> = Vec::new();
    for i in data{
        let p1 = PolarPoint { point : (i.0, i.1) };
        polar_data.push(p1);
    }
    let cartesian_data = to_cartesian(polar_data);
    cartesian_data
}
