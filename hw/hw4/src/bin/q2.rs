use std::num::ParseIntError;

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let ans_arg: Vec<String> = if args.len() > 1 { args[1..].to_vec() } else { Vec::new() };

    
    let ans: Vec<i32> = ans_arg.iter().map(|x| x.parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()
        .unwrap_or_else(|_| {
            println!("Enter a numeric score value");
            std::process::exit(1);
    });


    let converted_ans = make_grades(ans);
    // let converted_ans = make_grades2(ans);
    for i in converted_ans{
        print!("{} ", i);
    }
}

//q2.1
fn make_grades(ans: Vec<i32>) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    for element in ans{
        if element > 100 || element < 0{
            result.push("Invalid score".to_string());
        }else if element >= 95 {
            result.push("Excellent with A+".to_string());
        }else if element >= 81 {
            result.push("A".to_string());
        }else if element >= 71 {
            result.push("B".to_string());
        }else if element >= 61 {
            result.push("C".to_string());
        }else if element >= 50 {
            result.push("D".to_string());
        }else if element >= 0 {
            result.push("Failed with F".to_string());
        }
    }
    result
    
}


//q2.2
fn make_grades2(ans: Vec<i32>) -> Vec<String>{
    
    if ans.is_empty() {
        Vec::new()
    }else{
        let mut result: Vec<String> = make_grades2((&ans[1..]).to_vec());
        let element = ans[0];
        if element > 100 || element < 0{
            result.insert(0,"Invalid score".to_string());
        }else if element >= 95 {
            result.insert(0,"Excellent with A+".to_string());
        }else if element >= 81 {
            result.insert(0,"A".to_string());
        }else if element >= 71 {
            result.insert(0,"B".to_string());
        }else if element >= 61 {
            result.insert(0,"C".to_string());
        }else if element >= 50 {
            result.insert(0,"D".to_string());
        }else if element >= 0 {
            result.insert(0,"F".to_string());
        }
        result
    }

  
}