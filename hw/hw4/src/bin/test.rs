fn main(){
    // for i in 0..1{
    //     print!("hello");
    // }

  
    let multiline_string = "
        This is a
        multiline
        string.
    ";

    // Replace newline characters with spaces
    let single_line_string: String = multiline_string.replace("\n", " ");

    println!("{}", single_line_string);
    
    
    // star((5*2)-1, 5);
}


//3
// fn star(dot: i32, fixed: i32){
//     if dot <= 0{
//         return;
//     }
//     let line = if dot >= fixed{ (fixed*2) - dot } else { dot };

//     for _ in 0..line{
//         print!("*");
//     }
//     println!();
//     star( dot - 1, fixed);
    
// }

// fn star2(dot: i32, fixed: i32){
//     if dot <= 0{
//         return;
//     }
//     let line = if dot >= fixed{ 
//                         (fixed*2) - dot
//                     } else { 
//                         dot
//                     };

//     for _ in 0..(fixed - line){
//         print!(" ");
//     }
//     for _ in 0..line{
//         print!("*");
//     }
//     println!();
//     star2( dot - 1, fixed);
    
// }