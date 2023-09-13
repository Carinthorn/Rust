use assert_cmd::Command;
use assert_cmd::prelude::*;




//input string 
//q3.1
pub fn fahr_to_cel_v(){
    let args: Vec<String> = std::env::args().collect();
    let start = match args[1].parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid value for 'start'");
            std::process::exit(1);
        }
    };

    let end: f32 = match args[2].parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid value for 'end'");
            std::process::exit(1);
        }
    };

    let delta = match args[3].parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid value for 'delta'");
            std::process::exit(1);
        }
    };

    println!("<table>");
    println!("{:2}<tr>", "");
    println!("{:4}<th>Fah</th>", "");
    println!("{:4}<th>Celcius</th>", "");
    println!("{:2}</tr>", "");
    
    if start < end{
        for i in (start as i32..=end as i32).step_by(delta as usize){
            let celcius: f32 = (5.0/9.0)*(i as f32 - 32.0);
            println!("{:2}<tr>", "");
            println!("{:4}<th>{}</th>", "", i);
            println!("{:4}<th>{}</th>", "", format!("{:.1}", celcius));
            println!("{:2}</tr>", "");
            
        }
    }else if start > end{
        for i in (end as i32..=start as i32).rev().step_by(delta as usize){
            let celcius: f32 = (5.0/9.0)*(i as f32 - 32.0);
            println!("{:2}<tr>", "");
            println!("{:4}<th>{}</th>", "", i);
            println!("{:4}<th>{}</th>", "", format!("{:.1}", celcius));
            println!("{:2}</tr>", "");
        }
    }else{
        let celcius: f32 = (5.0/9.0)*(start - 32.0);
        println!("{:2}<tr>", "");
        println!("{:4}<th>{}</th>", "", start );
        println!("{:4}<th>{}</th>", "", format!("{:.1}", celcius));
        println!("{:2}</tr>", "");
    }

    println!("</table>");
}

//q3.2
pub fn power_taber(){
    let args: Vec<String> = std::env::args().collect();
    let x = match args[1].parse::<f32>(){
        Ok(value) => value, 
        Err(_) => {
            println!("Error: Invalid value for 'x'");
            std::process::exit(1);
        }
    };
    println!("<table>");
    println!("{:2}<tr>", "");
    println!("{:4}<th>x</th>", "");
    println!("{:4}<th>x^2</th>", "");
    println!("{:4}<th>x^3</th>", "");

    println!("{:2}</tr>", "");
    println!("{:2}<tr>", "");
    println!("{:4}<th>{}</th>", "", x);
    println!("{:4}<th>{}</th>", "", x.powf(2.0));
    println!("{:4}<th>{}</th>", "", x.powf(3.0));
    println!("{:2}</tr>", "");
    println!("</table>");


}