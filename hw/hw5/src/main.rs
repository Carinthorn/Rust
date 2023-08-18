use::hw5::{count_vowels_r, extract_quoted_words, split_grades};

fn main() {
    // let text: String = "Hello, world!".to_string();
    // let sliced_string = &text[1..];
    // print!("{}", sliced_string)

    // let text = extract("C ** *C++* *Java *Python* Rust*");
    // let result = extract_quoted_words("0"); //Rust*");
    // for i in result{
    //     println!("{}", i);
    // }

    let r = split_grades(vec!["H"]);
    // let str = "h123Aeijklj";
    // print!("{}", &str[0..1])
    // for i in text{
    //     println!("{}", i);
    // }

    
}


//Workkkkk
fn extract(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    fn extract_recursive<'a>(text: &'a str, index: usize, mut result2:  Vec<String>) -> Vec<String>{
        let words:Vec<String> = text.split_whitespace().map(|s: &str| s.to_string()).collect();
        if index == words.len() - 1 {
            return result2;
        }else{
            let mut count: usize = 0;
            let mut last: Option<usize> = None;
            for (j, c) in words[index].chars().enumerate(){
                if c == '*'{
                    count += 1;
                    if count < 2 && j == words.len() - 1{
                        result2.push("".to_string());
                    }else{
                        match last{
                            Some(last_index) => {
                                result2.push(words[index][last_index+1..j].to_string());
                                last = None
                            }
                            None => {
                                last = Some(j);
                            }
                        }
                    }
                }

            }
        }
        extract_recursive(text, index + 1, result2)
    }

    let result_temp: Vec<String> = extract_recursive(text, 0, result);
    // let result_new: Vec<&str> = result_temp.iter().map(|s| s.as_str()).collect();

// Now you can use result_new as needed

    return result_temp

}









