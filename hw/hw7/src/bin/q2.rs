//q2.1
fn main(){
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let mut inp: Vec<(f32, f32)> =  Vec::new();
    
    if args.len() > 2 {
        let length = if args.len() % 2 == 0 { args.len() - 1 } else { args.len() };
        
        for i in (1..length).step_by(2) { // 0 2 4
            let f1_result: Result<f32, std::num::ParseFloatError> = args[i].parse::<f32>();
            let f2_result = args[i + 1].parse::<f32>();

            match (f1_result, f2_result) {
                (Ok(f1), Ok(f2)) => inp.push((f1, f2)),
                (Err(_), _) | (_, Err(_)) => {
                    eprintln!("Enter valid integers");
                    std::process::exit(1)
                }
            }
        }
    }else if args.len() <= 2 {
        eprintln!("Need more than 1 input");
        std::process::exit(1);
    }
    let result: Vec<Vec<(f32, f32)>> = sort_point(inp);
    // let result: Vec<Vec<(f32, f32)>> = sort_point2(inp);
    for i in result{
        println!("{:?}", i);
    }
    
}


pub fn sort_point(mut inp : Vec<(f32, f32)>) -> Vec<Vec<(f32, f32)>>{
    let mut result = Vec::new();
    //asceding
    inp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result.push(inp.clone());
    //desceding
    inp.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    result.push(inp);

    return result 
}


//q2.2
pub fn sort_point2(mut inp : Vec<(f32, f32)>) -> Vec<Vec<(f32, f32)>>{
    let mut result = Vec::new();

    //ascending
    for i in 0..inp.len(){
        let mut swapped = false;
        for j in 0..(inp.len() - i - 1){
            if inp[j].0 > inp[j + 1].0{
                inp.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false{
            break;
        }
    }
    result.push(inp.clone());

    for i in 0..inp.len(){
        let mut swapped = false;
        for j in 0..(inp.len() - i - 1){
            if inp[j].1 < inp[j + 1].1{
                inp.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false{
            break;
        }
    }
    result.push(inp.clone());

    return result;
}