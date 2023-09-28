mod lib; 
use hw9::read_csv;
use rand::thread_rng;
use lib::{cal_average_area, Layer, Circle, layers_save_csv, gen_obj_layer_list, save_data};
use std::error::Error;
fn main() {
    let mut rng = thread_rng();
    //create and save n list of layer to output.csv file 
    let _save_layer: Result<(), Box<dyn Error>> = layers_save_csv(10, "output_original", "csv"); 

    //generate 10 layers and calculate average area of each layer then save to output_avg.csv file
    let binding = gen_obj_layer_list(&mut rng, 10);
    let cal_avg = cal_average_area(&binding);
    let _save_cal_avg = save_data(cal_avg, None, "output_avg", "csv");

    //read output.csv file then find avg then save to output2.csv file 
    let _output: Result<(), Box<dyn Error>> = read_csv("output_original.csv", "output_csv", "csv");

    //read output.csv file then generate the html version of it with min and max
    let _save_cal_avg: Result<(), Box<dyn Error>> = read_csv("output_original.csv", "output_html", "html");
}
