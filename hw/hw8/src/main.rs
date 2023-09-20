use Pt2::{read_csv_cartesian, save_data, read_csv_polar, to_polar, to_cartesian};
mod Pt2; 

fn main() {

    let polar_data = read_csv_cartesian("input_car.csv");
    match save_data("output_polar", ".csv", polar_data) {
        Ok(()) => println!("Successfully saved"),
        Err(_) => println!("Error"),
    } 

    let cartesian_data = read_csv_polar("output_polar.csv");
    match save_data("output_car", ".csv",cartesian_data) {
        Ok(()) => println!("Successfully saved"),
        Err(_) => println!("Error"),
    }

    let polar_data = read_csv_cartesian("input_car.csv");
    match save_data("output_polar", ".html", polar_data) {
        Ok(()) => println!("Successfully saved"),
        Err(_) => println!("Error"),
    } 

    //problme
    let cartesian_data = read_csv_polar("output_polar.csv");
    match save_data("output_car", ".html",cartesian_data) {
        Ok(()) => println!("Successfully saved"),
        Err(_) => println!("Error"),
    }

}
