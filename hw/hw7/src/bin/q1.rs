//q1.1

fn main(){
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let inp: Vec<i32> =  if args.len() > 1{
        args[1..].iter()
            .map(|x: &String| x.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .unwrap_or_else(|err|{
                eprintln!("Enter valid integers");
                std::process::exit(1);
            })
    } else {
        Vec::new()
    };
    // let result: Vec<Vec<i32>> = sort_values(inp);
    let result = sort_value2(inp);
    for i in result{
        println!("{:?}", i);
    }
}

fn sort_values(mut inp: Vec<i32>) -> Vec<Vec<i32>>{
    let mut result = Vec::new();
    inp.sort();
    result.push(inp.clone());

    inp.sort_by(|a, b| b.cmp(a));
    result.push(inp);
    return result
}

//q1.2
fn sort_value2(mut inp: Vec<i32>) -> Vec<Vec<i32>>{
    let mut result = Vec::new();
    let n = inp.len();
    for i in 0..n {
        let mut swapped = false; 
        for j in 0..n-i-1 {
            if inp[j] > inp[j + 1]{
                inp.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false{
            break;
        }
    }
    result.push(inp.clone());

    let mut descending = Vec::new();
    for i in inp{
        descending.insert(0, i);
    }
    result.push(descending);
    return result
}
