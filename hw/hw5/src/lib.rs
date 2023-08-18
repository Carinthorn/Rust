
fn count_vowels(text: &str) -> i32{
    let vowel = vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    let mut count = 0;

    for i in text.chars(){
        if vowel.contains(&i.to_string().as_str()){
            count += 1;
        }
    }
    count
}

pub fn count_vowels_r(text: &str) -> i32{ //error
    let vowel = vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    if text.len() > 0{
        let slice = &text[1..];
        let mut num = count_vowels_r(slice);
        if vowel.contains(&&text[0..1]){
            num += 1;
        }
        return num
    }else{
        return 0
    }
}

fn count_vowels_v2(text: &str) -> Vec<(String, i32)>{
    let mut result = Vec::new();
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    for i in words{
        result.push((i.clone(), count_vowels(i.as_str())));
    }
    result
}


//2
//input empty string , input number, input other character
pub fn split_grades(input: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut fail: Vec<&str> = Vec::new();
    let mut pass: Vec<&str> = Vec::new();
    
    for i in input {
        let n = i.to_uppercase();
        if n == "D".to_string() || n == "F".to_string() {
            fail.push(i);
        } else if n == "A+".to_string() || n == "A".to_string() || n == "B".to_string() || n == "C".to_string() {
            pass.push(i);
        } 
    }
    (pass.clone(), fail.clone())
}

//enter string or other char
fn split_scores(input:Vec<i32>) -> (Vec<(&'static str, i32)>, Vec<(&'static str, i32)>){
    let mut pass: Vec<(&str, i32)> = Vec::new();
    let mut fail: Vec<(&str, i32)> = Vec::new();
    for i in input{
        if i < 101{
            if i >= 95 {
                pass.push(("A+", i));
            }else if i >= 81 {
                pass.push(("A", i));
            }else if i >= 71 {
                pass.push(("B", i));
            }else if i >= 61 {
                pass.push(("C", i));
            }else if i >= 50 {
                fail.push(("D", i));
            }else if i >= 0 {
                fail.push(("F", i));
            }
        }
    }
    (pass, fail)
}

//3
//input **python*, 0
pub fn extract_quoted_words(text: &str) -> Vec<String>{
    // *Python*
    let mut last = None;
    let mut count = 0;
    let words:Vec<String> = text.split_whitespace().map(|s: &str| s.to_string()).collect();
    let mut result:Vec<String> = Vec::new();

    for word in words{
       for (j, c) in word.chars().enumerate(){
            if c == '*' {
                count += 1;
                if word.len() >= 2 && count < 2 && j == word.len()-1{
                    result.push("".to_string());
                    // last = None
                }else{
                    match last{
                        Some(last_index) => {
                            result.push(word[last_index+1..j].to_string());
                            last = None
                        }
                        None => {
                            last = Some(j);
                        }
                    }
                }
            }else if j == word.len() - 1{
                last = None
            }
        }
    }
    result
}


//"C ** *C++* *Java *Python* Rust* **word**"
fn extract_quoted_words_r(text: &str) -> Vec<String> {
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
    if text.len() > 1{
        let result_temp: Vec<String> = extract_recursive(text, 0, result);
        return result_temp
    }else{
        return result
    }
    // let result_new: Vec<&str> = result_temp.iter().map(|s| s.as_str()).collect();
    

}




#[cfg(test)]
mod test{
    use std::vec;

    use super::{count_vowels, count_vowels_v2, count_vowels_r, split_grades, split_scores, extract_quoted_words, extract_quoted_words_r};
    // #[test]
    fn test_vowels_count1() {
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("abEcd"), 2);
        assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    }

    // #[test]
    fn test_count_vowels_r(){
        assert_eq!(count_vowels_r(""), 0);
        assert_eq!(count_vowels_r("abEcd"), 2);
        assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
    }

    // #[test]
    fn test_vowels_count2() {
        assert_eq!(count_vowels_v2(""), []);
        assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1) // 'U'
        ]
        );
    }

    //2
    // #[test]
    fn test_split_grades(){
        assert_eq!(split_grades(vec![""]), (vec![], vec![]));
        assert_eq!(split_grades(vec!["H"]), (vec![], vec![]));
        assert_eq!(split_grades(vec!["12","2323"]), (vec![], vec![]));
        assert_eq!(split_grades(vec!["B", "F", "A+", "D", "C"]), (vec!["B", "A+", "C"], vec!["F", "D"]));
        assert_eq!(split_grades(vec!["B", "F", "a+", "d", "C"]), (vec!["B", "a+", "C"], vec!["F", "d"]));
        assert_eq!(split_grades(vec!["B", " ", "1", "hdj", "C"]), (vec!["B","C"], vec![]));
    }

    // #[test]
    fn test_split_scores(){
        assert_eq!(split_scores(vec![]),  (vec![], vec![]));
        assert_eq!(split_scores(vec!["A", "B"]),  (vec![], vec![]));
        assert_eq!(split_scores(vec![75, 42, 98, 54, 63]),  (vec![("B",75), ("A+", 98), ("C", 63)], vec![("F", 42), ("D", 54)]));
        assert_eq!(split_scores(vec![-1, 42, 105, 54, 63]),  (vec![("C", 63)], vec![("F", 42), ("D", 54)]));
    }

    // #[test]
    fn test_extract_quoted_words() {
        assert_eq!(extract_quoted_words(""), Vec::<String>::new());
        assert_eq!(extract_quoted_words_r("0"),Vec::<String>::new());
        assert_eq!(extract_quoted_words("C ** *C++* *Java *Python* Rust*"),vec!["", "C++", "Python"]); 
        assert_eq!(extract_quoted_words_r("C ** n*C++* *Java *Python*hj Rust*"),vec!["", "C++", "Python"]);

    }


    // #[test]
    fn test_extract_quoted_words_r(){
        assert_eq!(extract_quoted_words_r(""),  Vec::<String>::new());
        assert_eq!(extract_quoted_words_r("0"),Vec::<String>::new());
        assert_eq!(extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),vec!["", "C++", "Python"]);
        assert_eq!(extract_quoted_words_r("C ** n*C++* *Java *Python*hj Rust*"),vec!["", "C++", "Python"]);
    }

}