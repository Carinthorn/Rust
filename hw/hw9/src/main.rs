mod lib; 
use hw9::read_csv;
use lib::{cal_average_area, Layer, Circle, layers_save_csv};

fn main() {
    // let result = cal_average_area(&vec![ 
    //     Layer{ 
    //         name: "Layer 1".to_string(), 
    //         color: "#000000".to_string(), 
    //         circles: vec![
    //             Circle{point: (0,0), radius: 10}, 
    //             Circle{point: (0,0), radius: 20}]}, 
    //     Layer{ 
    //         name: "Layer 2".to_string(), 
    //         color: "#000000".to_string(), 
    //         circles: vec![
    //             Circle{point: (0,0), radius: 5}, 
    //             Circle{point: (0,0), radius: 10}]}
    //     ]);
    // let r = layers_save_csv(2, "output", "csv"); //but need to eliminate quote at the end
    let n = read_csv("output.csv", "output2");
    
}
