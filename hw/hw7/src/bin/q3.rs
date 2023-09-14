
//input string 
fn main(){
    fahr_to_cel_v();
    // power_taber();
}

//q3.1
pub fn fahr_to_cel_v(){
    //200 10 0
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Error: Invalid number of arguments");
        std::process::exit(1);
    }

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
        Ok(value) => value.abs(),
        Err(_) => {
            eprintln!("Error: Invalid value for 'delta'");
            std::process::exit(1);
        }
    };

    if delta == 0.0 {
        println!("enter delta > 0");
        std::process::exit(1);
    }

    println!("{}",
    r#"<style>
table, td {
  border: 1px solid #000000;
  border-collapse: collapse;
}
</style>
    "#);

    println!("<table>");
    println!("{:2}<tr>", "");
    println!("{:4}<td>Fah</td>", "");
    println!("{:4}<td>Celcius</td>", "");
    println!("{:2}</tr>", "");
    
 
    let (start, end, step) = if start < end {
        (start, end, delta)
    } else if start > end {
        (end, start, delta)
    } else {
        (start, end, delta)
    };

    for i in (start as i32..=end as i32).step_by(step as usize) {
        let celcius: f32 = (5.0 / 9.0) * (i as f32 - 32.0);
        println!("{:2}<tr>", "");
        println!("{:4}<td>{}</td>", "", i);
        println!("{:4}<td>{}</td>", "", format!("{:.1}", celcius));
        println!("{:2}</tr>", "");
    }

    println!("</table>");
}

//q3.2
pub fn power_taber(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2{
        eprintln!("Error: Invalid number of arguments");
        std::process::exit(1);
    }
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
        Ok(value) => value.abs(),
        Err(_) => {
            eprintln!("Error: Invalid value for 'delta'");
            std::process::exit(1);
        }
    };

    if delta == 0.0 {
        println!("enter delta > 0");
        std::process::exit(1);
    }

    println!("{}",
    r#"<style>
table, td {
  border: 1px solid #000000;
  border-collapse: collapse;
}
</style>
    "#);

    println!("<table>");
    println!("{:2}<tr>", "");
    println!("{:4}<td>x</td>", "");
    println!("{:4}<td>x^2</td>", "");
    println!("{:4}<td>x^3</td>", "");
    println!("{:2}</tr>", "");

    let (start, end, step) = if start <= end {
        (start, end, delta)
    } else {
        (end, start, -delta)
    };

    for i in (start as i32..=end as i32).step_by(step as usize) {
        println!("{:2}<tr>", "");
        println!("{:4}<td>{}</td>", "", i);
        println!("{:4}<td>{}</td>", "", i.pow(2));
        println!("{:4}<td>{}</td>", "", i.pow(3));
        println!("{:2}</tr>", "");
    }

    println!("</table>");
}