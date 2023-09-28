
//1.1 test case
use rand::{Rng, thread_rng};
use csv::{ReaderBuilder, WriterBuilder, Reader};
use std::{fs::File, error::Error, f32::consts::PI};
use std::f32::INFINITY;


#[derive(Clone, Debug)]
pub struct Circle{
    pub point: (i32, i32), 
    pub radius: i32
}

#[derive(Clone, Debug)]
pub struct Layer{
    pub name: String, 
    pub color: String, 
    pub circles: Vec<Circle>
}

pub trait Data{
    fn get_data(&self) -> String;
}

impl Data for (&str, f32){
    fn get_data(&self) -> String{
        format!("{},{}", self.0, self.1)
    }
}

impl Data for Layer{
    fn get_data(&self) -> String{
        let mut result = String::new();
        result += &(format!("{},{},", self.name, self.color));
        for circle in &self.circles{
            result += &(format!("{},{},{},", circle.point.0, circle.point.1, circle.radius))
        }
        result
    }
}

//1.1
pub fn gen_obj_layer_list(rng: &mut impl Rng, n: i32) -> Vec<Layer>{
    let mut result = Vec::new();
    for i in 1..=n{
        let name = format!("Layer {i}");

        let rr = format!("{:02X}",rng.gen_range(0..=255));
        let gg = format!("{:02X}", rng.gen_range(0..=255));
        let bb = format!("{:02X}", rng.gen_range(0..=255));
        let aa = format!("{:02X}", rng.gen_range(0..=255));
        let color = format!("#{rr}{gg}{bb}{aa}");

        let num = rng.gen_range(20..=50);
        let mut circles: Vec<Circle> = Vec::new();
        for _ in 0..num{
            let x = rng.gen_range(-100..=100);
            let y = rng.gen_range(-100..=100);
            let radius = rng.gen_range(-10..=20);
            circles.push(Circle{point: (x,y), radius});
        }

        result.push(Layer{ 
            name, 
            color, 
            circles
        })
    }
    result
}

//help
#[test]
fn test_gen_obj_layer_list(){
    let mut rng = thread_rng();
    let result = gen_obj_layer_list(&mut rng, 10);
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].name, "Layer 1");
    assert_eq!(result[9].name, "Layer 10");
    assert!(result[0].circles.len() >= 20 && result[0].circles.len() <= 50);
    assert!(result[0].circles[0].point.0 >= -100 && result[0].circles[0].point.0 <= 100);
    assert!(result[0].circles[0].point.1 >= -100 && result[0].circles[0].point.1 <= 100);
    assert!(result[0].circles[0].radius >= -10 && result[0].circles[0].radius <= 20);
}

//1.2
pub fn cal_average_area(layers: &Vec<Layer>) -> Vec<(&str, f32)>{
    let mut result = Vec::new();
    for layer in layers{
        let mut sum = 0.0;
        for circle in &layer.circles{ 
            sum += circle.radius as f32 * circle.radius as f32 * PI;
        }
        let average = sum / layer.circles.len() as f32;
        result.push((layer.name.as_str(), average));
    }
    result 
}

#[test]
fn test_cal_average_area(){
    let mut rng = thread_rng();
    let layers = gen_obj_layer_list(&mut rng, 10);
    let result = cal_average_area(&layers);
    assert_eq!(result.len(), 10);

    assert_eq!(cal_average_area(
        &vec![ Layer{ 
            name: "Layer 1".to_string(), 
            color: "#000000".to_string(), 
            circles: vec![
                Circle{point: (0,0), radius: 10}, 
                Circle{point: (0,0), radius: 20}]}]),vec![("Layer 1", 785.3982)]);

    assert_eq!(cal_average_area(
        &vec![ 
            Layer{ 
                name: "Layer 1".to_string(), 
                color: "#000000".to_string(), 
                circles: vec![
                    Circle{point: (0,0), radius: 10}, 
                    Circle{point: (0,0), radius: 20}]}, 
            Layer{ 
                name: "Layer 2".to_string(), 
                color: "#000000".to_string(), 
                circles: vec![
                    Circle{point: (0,0), radius: 5}, 
                    Circle{point: (0,0), radius: 10}]}
            ]),vec![("Layer 1", 785.3982), ("Layer 2", 196.34955)]);
}

//2.1
pub fn save_data<T: Clone + Data>(data: Vec<T>, layers: Option<Vec<Layer>>,  filename: &str, filetype: &str) -> Result<(), Box<dyn Error>>{
    let file = File::create(filename.to_owned() + "." + filetype)?;
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(file);
    match filetype{
        "csv" => {
            for i in data{
                writer.write_record(&[i.get_data()])?;
            }
        }
        "html" => {
            let html = convert_to_html(data, layers);
            writer.write_record(&[html])?;
        }
        _ => {
            println!("Invalid file type");
            return Ok(());
        }
    }
    Ok(())
}
 
pub fn load_data(filename: &str) -> Result<Vec<Layer>, Box<dyn Error>>{
    //handle 
    if !filename.ends_with(".csv"){
        println!("Invalid file type");
        return Err("Invalid file type".into());
    }

    let file = File::open(filename).map_err(|e| {
        println!("Error opening file: {}", e);
        Box::new(e) as Box<dyn Error>
    });
    let file = file.unwrap();

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .has_headers(false)
        .from_reader(file);
    let mut result: Vec<Layer> = Vec::new();
    for data in reader.records(){
        if let Ok(rec) = data {
            let name: String = rec[0].to_string();
            let color: String = rec[1].to_string();
            let mut circles: Vec<Circle> = Vec::new();
            for i in (2..rec.len()-2).step_by(3) {
                let x: i32 = rec[i].parse()?;
                let y: i32 = rec[i+1].parse()?;
                let radius: i32 = rec[i+2].parse()?;

                if x != 101 || y != 101 || radius != 101{
                    let circle = Circle{point: (x,y), radius};
                    circles.push(circle);
                }
            }
            result.push(Layer{name, color, circles});
        }
    }
    Ok(result)
}


fn convert_to_html<T: Data>(data: Vec<T>, layers: Option<Vec<Layer>>) -> String{ //set layer to be optional 
    let mut result = String::new();
    result.push_str(&format!("{}",
r#"<style>
table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
    font-family: Arial, sans-serif;
}
</style>
"#));
    result.push_str("<table>\n");
    result.push_str(&format!("{:2}<tr>\n", ""));
    result.push_str(&format!("{:4}<td>Layer</td>\n", ""));
    result.push_str(&format!("{:4}<td>Average area of circles</td>\n", ""));
    result.push_str(&format!("{:4}<td>Min area</td>\n", ""));
    result.push_str(&format!("{:4}<td>Max area</td>\n", ""));
    result.push_str(&format!("{:2}</tr>\n", ""));

    for (index, i) in data.iter().enumerate(){
        let dt = i.get_data();
        let parts = dt.split(",").collect::<Vec<&str>>();
        let x = parts[0];
        let y = parts[1];
        result.push_str(&format!("{:2}<tr>\n", ""));
        result.push_str(&format!("{:4}<td>{}</td>\n", "", x));
        result.push_str(&format!("{:4}<td>{}</td>\n", "", y));
        
        match layers {
            Some(ref layer) => {
                let pair = find_max_min(layer.to_vec());
                result.push_str(&format!("{:4}<td>{}</td>\n", "", pair[index].0));
                result.push_str(&format!("{:4}<td>{}</td>\n", "", pair[index].1));
                result.push_str(&format!("{:2}<tr>\n", ""));
            }
            None => {
                result.push_str(&format!("{:2}<tr>\n", ""));
                continue;
            }
        }
        result.push_str(&format!("{:2}<tr>\n", ""));
    }
    result.push_str("<table>\n");
    result
}


pub fn find_max_min(data: Vec<Layer>) -> Vec<(f32, f32)>{
    let mut partner: Vec<(f32, f32)> = Vec::new();

    for i in 0..data.len(){ //2
        let mut max = -INFINITY;
        let mut min = INFINITY;

        for j in 0..data[i].circles.len(){
            let circle = &data[i].circles[j];
            let area = circle.radius as f32 * circle.radius as f32 * PI;
            if area > max{
                max = area;
            }
            if area < min{
                min = area;
            }
        }
        partner.push((min, max));
    }
    partner
}

//2.1
pub fn layers_save_csv(n: i32, filename: &str, filetype: &str) -> Result<(), Box<dyn Error>>{
    let mut rng = thread_rng();
    let data = gen_obj_layer_list(&mut rng, n);
    let success = save_data(data, None,  filename, filetype);
    match success {
        Ok(_) => println!("Save data successfully"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

//2.2
pub fn read_csv(filename: &str,  output_file: &str,  filetype: &str) -> Result<(), Box<dyn Error>>{
    let current_dir = std::env::current_dir()?;
    let full_path = current_dir.join(filename);
    if !full_path.exists(){
        println!("Error: File not found : {:?}", full_path);
        return Err("File not found".into());
    }

    let layers: Vec<Layer> = load_data(&full_path.to_string_lossy()).unwrap();
    let result: Vec<(&str, f32)> = cal_average_area(&layers);
    let cloned_layers = layers.clone();
    let mut success: Result<(), Box<dyn Error>> = Ok(());
    if filetype == "csv"{
        success = save_data(result, None, output_file, "csv");
    }else if filetype == "html"{
        success = save_data(result, Some(cloned_layers), output_file, "html");
    }
    match success {
        Ok(_) => println!("Save data successfully"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}



