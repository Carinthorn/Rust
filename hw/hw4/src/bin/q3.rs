// cargo run --bin q3 line_number
// cargo test --test test_arr2


fn main(){
    let args: Vec<String> = std::env::args().collect();
    let line_args = if args.len() < 2{ "" } else { &args[1] };
    let line: i32 = match line_args.parse::<i32>(){
        Ok(num) => num.abs(), 
        Err(_) => {
            print!("Enter a numeric value");
            std::process::exit(1);
        }
    }; 
    //q3.1
    // make_arrow1(line);
    //q3.2
    // make_arrow2(line);
    
    //q3.3
    // make_arrow1_recur(line);
    // make_arrow2_recur(line);
}


fn make_arrow1(line: i32){
    for i in 1..(line*2+1){
        if i > line{
            for _ in (1..=(line*2-(i))).rev(){
                print!("*");
            }
            if i == line*2{
                std::process::exit(1);
            }
            println!("{}", ("").trim_end());
        }else{
            for _ in 1..(i+1){
                print!("*");
            }
            println!("{}", ("").trim_end());
        }
    }
}

fn make_arrow1_recur(line: i32){
    star1((line*2)-1, line);
}

fn star1(dot: i32, fixed: i32){
    if dot <= 0{
        return;
    }
    let line = if dot >= fixed{ 
                        (fixed*2) - dot
                    } else { 
                        dot
                    };

    for _ in 0..line{
        print!("*");
    }
    println!();
    star1( dot - 1, fixed);
    
}

pub fn make_arrow2(line: i32){ 
    for i in 0..(line*2){ //0-5
        if i > line - 1{ // 3 4 5
            for _ in 0..(i - line + 1){ // 
                print!(" ");
            }
            for _ in (0..(line*2 - i - 1)).rev(){  
                print!("*");
            }

            println!();
            if i == (line*2)-2{
                return;
            }
            
        }else{
            for _ in 0..(line - 1 -i){ 
                print!(" ");
            }
            for _ in 0..(i+1){
                print!("*");
            }
            println!("{}", ("").trim_end());
        }
    }
}

fn make_arrow2_recur(line: i32){
    star2((line*2)-1, line);
}


fn star2(dot: i32, fixed: i32){
    if dot <= 0{
        return;
    }
    let line = if dot >= fixed{ 
                        (fixed*2) - dot
                    } else { 
                        dot
                    };

    for _ in 0..(fixed - line){
        print!(" ");
    }
    for _ in 0..line{
        print!("*");
    }

    if dot != -2{ 
        println!();
    }
    
    // println!();
    star2( dot - 1, fixed);
    
}


