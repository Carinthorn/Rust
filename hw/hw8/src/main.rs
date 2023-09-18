use Pt::{read_csv_cartesian, save_data, read_csv_polar};

mod Pt; 
// Correct use statement

fn main() {
    // let polar_data = read_csv_cartesian("input.csv");
    // match save_data("output_polar", ".csv", polar_data) {
    //     Ok(()) => println!("Successfully saved"),
    //     Err(_) => println!("Error"),
    // } 

    // let cartesian_data = read_csv_polar("output_polar.csv");
    // match save_data("output_polar", ".csv",cartesian_data) {
    //     Ok(()) => println!("Successfully saved"),
    //     Err(_) => println!("Error"),
    // }

    //html
    // let polar_data = read_csv_cartesian("input.csv");
    // match save_data("output_polar", ".html", polar_data) {
    //     Ok(()) => println!("Successfully saved"),
    //     Err(_) => println!("Error"),
    // } 

    //problme
    let cartesian_data = read_csv_polar("input_car.csv");
    match save_data("output_polar", ".html",cartesian_data) {
        Ok(()) => println!("Successfully saved"),
        Err(_) => println!("Error"),
    }

}
