fn main(){
    let args: Vec<String> = std::env::args().collect();
    print!("{}", args.len());
    // if args.len() < 4 || args.len() > 4{
    //     print!("Input: <start_f> <end_f> <delta>")
    //     return;
    // }

    let start: f32 = match args[1].parse::<f32>(){
        Ok(num) => num, 
        Err(_) => {
            print!("Input a numeric value for start");
            std::process::exit(1)
        }
    };

    let end:f32 = match args[2].parse::<f32>(){
        Ok(num) => num, 
        Err(_) => {
            print!("Input a numeric value for end");
            std::process::exit(1)
        }
    };

    let start:f32 = match args[3].parse::<f32>(){
        Ok(num: f32) => num, 
        Err(_) => {
            print!("Input a numeric value for delta");
            std::process::exit(1)
        }
    };

    // fahr_to_cel_v(start, end, delta);

    

}

//blank input, str input, start less than end, start more than end, indivisible delta, negative delta, 
fn fahr_to_cel_v(start:f32, end:f32, delta:f32){
    unimplemented!();
}