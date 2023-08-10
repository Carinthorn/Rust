// cargo run --bin q1 f_start f_end delta
// cargo test --test test_q1
fn main(){
    let args: Vec<String> = std::env::args().collect(); 
   
    if args.len() < 4 || args.len() > 4{
        print!("Input: <start_f> <end_f> <delta>");
        return;
    }

    let mut start: f32 = match args[1].parse::<f32>(){
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

    let delta:f32 = match args[3].parse::<f32>(){
        Ok(num) => num.abs(), 
        Err(_) => {
            print!("Input a numeric value for delta");
            std::process::exit(1)
        }
    };
    println!("{:>6}\t{:>6}", "Fahr","Celcius");

    // fahr_to_cel_v(start, end, delta);
    fahr_to_cel_v_2(start, end, delta);
}

//q1.1
fn fahr_to_cel_v(start:f32, end:f32, delta:f32){
    
    if start < end{
        for i in (start as i32..=end as i32).step_by(delta as usize){
            let celcius: f32 = (5.0/9.0)*(i as f32 - 32.0);
            println!("{:>6}\t{:>7}", i , format!("{:.1}", celcius));
        }

    }else if start > end{
        for i in (end as i32..=start as i32).rev().step_by(delta as usize){
            let celcius: f32 = (5.0/9.0)*(i as f32 - 32.0);
            println!("{:>6}\t{:>7}", i, format!("{:.1}", celcius));
        }

    }else{
        let celcius: f32 = (5.0/9.0)*(start - 32.0);
        println!("{:>6}\t{:>7}", start, format!("{:.1}", celcius));
    }
}


//q1.2
fn fahr_to_cel_v_2(mut start:f32, end:f32, delta:f32){
    if start < end{
        let celcius: f32 = (5.0/9.0)*(start - 32.0);
        println!("{:>6}\t{:>7}", start, format!("{:.1}", celcius));
        start = start + delta;
        if start > end{
            std::process::exit(1)
        }else{
            fahr_to_cel_v_2(start, end, delta)
        }
    }else if start > end{
        let celcius: f32 = (5.0/9.0)*(start - 32.0);
        println!("{:>6}\t{:>7}", start, format!("{:.1}", celcius));
        start = start - delta;
        if start < end{
            std::process::exit(1)
        }else{
            fahr_to_cel_v_2(start, end, delta)
        }

    }else{
        let celcius: f32 = (5.0/9.0)*(start - 32.0);
        println!("{:>6}\t{:>7}", start, format!("{:.1}", celcius));
    }
}