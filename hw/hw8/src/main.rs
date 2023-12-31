use pt2::{read_csv_cartesian, save_data, read_csv_polar, to_polar, to_cartesian, Point};
mod pt2; 

fn main() {
    let result = to_polar(vec![Point { point: (1.0, 1.0) }, Point { point: (2.0, 2.0) }]);
    println!("from cartesian to polar");
    for i in &result{
        println!("{:?}", i);
    }

    let result2 = to_cartesian(result);
    println!("from polar to cartesian");
    for i in result2{
        println!("{:?}", i);
    }

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
