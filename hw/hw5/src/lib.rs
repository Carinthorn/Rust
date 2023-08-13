
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

fn split_grades(input: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
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
//extract_quoted_words("C ** *C++* *Java *Python* Rust* **word**")
//["", "C++", "Python"]
// fn extract_quoted_words(text: &str) -> Vec<&str>{
//     let words:Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
//     let mut result:Vec<&str> = Vec::new();
//     for i in words{
//         if &i[0..1] == "*" && &i[i.len()-2..i.len()-1] == "*"{
//             result.push(i[1..i.len()-1]);
//         }
//     }
//     result
// }


#[cfg(test)]
mod test{
    use super::{count_vowels, count_vowels_v2, count_vowels_r, split_grades, split_scores,}; //extract_quoted_words};
    #[test]
    fn test_vowels_count1() {
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("abEcd"), 2);
        assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    }

    #[test]
    fn test_count_vowels_r(){
        assert_eq!(count_vowels_r(""), 0);
        assert_eq!(count_vowels_r("abEcd"), 2);
        assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
    }

    #[test]
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
    #[test]
    fn test_split_grades(){
        assert_eq!(split_grades(vec!["B", "F", "A+", "D", "C"]), (vec!["B", "A+", "C"], vec!["F", "D"]));
        assert_eq!(split_grades(vec!["B", "F", "a+", "d", "C"]), (vec!["B", "a+", "C"], vec!["F", "d"]));
        assert_eq!(split_grades(vec!["B", " ", "1", "hdj", "C"]), (vec!["B","C"], vec![]));

    }

    #[test]
    fn test_split_scores(){
        assert_eq!(split_scores(vec![75, 42, 98, 54, 63]),  (vec![("B",75), ("A+", 98), ("C", 63)], vec![("F", 42), ("D", 54)]));
        assert_eq!(split_scores(vec![-1, 42, 105, 54, 63]),  (vec![("C", 63)], vec![("F", 42), ("D", 54)]));
    }

    // #[test]
    // fn test_extract_quoted_words() {
    //     assert_eq!(extract_quoted_words(""), []);
    //     assert_eq!(extract_quoted_words("C ** *C++* *Java *Python* Rust*"),["", "C++", "Python"]);
    // }

}