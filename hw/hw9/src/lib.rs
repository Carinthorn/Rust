//find help kw

//1.1 test case
use rand::{Rng, thread_rng};
use csv::{ReaderBuilder, WriterBuilder, Reader};
use std::{fs::File, error::Error, f32::consts::PI};

pub struct Circle{
    pub point: (i32, i32), 
    pub radius: i32
}

pub struct Layer{
    pub name: String, 
    pub color: String, 
    pub circles: Vec<Circle>
}

trait Data{
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
fn gen_obj_layer_list(rng: &mut impl Rng, n: i32) -> Vec<Layer>{
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

    //layer name 

    //check if gen range > 20 and < 50

    //check if gen x, y > -100 and < 100
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
fn save_data<T: Data>(data: Vec<T>, filename: &str, filetype: &str) -> Result<(), Box<dyn Error>>{
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
            let html = convert_to_html(data);
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
    let file = File::open(filename)?;
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
            for i in (2..rec.len()).step_by(3) {
                let x:i32 = rec[i].parse()?;
                let y:i32 = rec[i+1].parse()?;
                let radius:i32 = rec[i+2].parse()?;
                let circle = Circle{point: (x,y), radius};
                circles.push(circle);
            } 
            result.push(Layer{name, color, circles});
        }
    }
    Ok(result)
}

//help
fn convert_to_html<T: Data>(data: Vec<T>) -> String{
    let mut result = String::new();
    result
}

//2.1
pub fn layers_save_csv(n: i32, filename: &str, filetype: &str) -> Result<(), Box<dyn Error>>{
    let mut rng = thread_rng();
    let data = gen_obj_layer_list(&mut rng, n);
    let success = save_data(data, filename, filetype);
    match success {
        Ok(_) => println!("Save data successfully"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

//2.2
pub fn read_csv(filename: &str, output_file: &str) -> Result<(), Box<dyn Error>>{
    let file = File::open(filename)?;
    let layers = load_data(filename).unwrap();
    let result = cal_average_area(&layers);
    let success = save_data(result, output_file, "csv");
    match success {
        Ok(_) => println!("Save data successfully"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}


