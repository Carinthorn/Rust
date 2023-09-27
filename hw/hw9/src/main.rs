mod lib; 
use hw9::read_csv;
use rand::thread_rng;
use lib::{cal_average_area, Layer, Circle, layers_save_csv, gen_obj_layer_list};
use std::error::Error;
fn main() {
    let r: Result<(), Box<dyn Error>> = layers_save_csv(10, "output", "csv"); 
    let n: Result<(), Box<dyn Error>> = read_csv("output.csv", "output2");
}
