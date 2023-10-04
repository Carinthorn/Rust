//1
pub fn vflip(img: &[String]) -> Vec<String>{
    let result: Vec<_> = img.iter().rev().cloned().collect();
    result
}

pub fn hflip(img: &[String]) -> Vec<String>{
    let max_len = img.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut new: Vec<String> = Vec::new();
    for i in img{
        if i.len() < max_len{
            let mut temp = i.clone();
            for _u in 0..(max_len - i.len()){
                temp.push(' ');
            }
            new.push(temp);
        }else{
            new.push(i.clone());
        }
    }
    let mut result = Vec::new();
    for i in new{
        result.push(i.chars().rev().collect());
    }
    result
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);

    let data = [
    "<--",
    "#####",
    "<=="
    ].map(|v| v.to_string());

    assert_eq!(
    vflip(&data), [
    "<==",
    "#####",
    "<--"
    ]);

    assert_eq!(
    hflip(&data), [
    "  --<",
    "#####",
    "  ==<"
    ]);
}

//2
pub fn vcat(img1: &[String], img2: &[String]) -> Vec<String>{
    let mut result = img1.clone().to_vec();
    result.extend_from_slice(img2);
    result
}

pub fn hcat(img1: &[String], img2: &[String]) -> Vec<String>{
    let mut result = Vec::new();
    let min_len = if img1.len() > img2.len() {img2.len()} else {img1.len()};
    let max_len1 = img1.iter().map(|v: &String| v.len()).max().unwrap_or(0);
    // let max_len2 = img2.iter().map(|v: &String| v.len()).max().unwrap();

    for i in 0..min_len{
        if img1[i].len() < max_len1{
            let mut temp = img1[i].clone();
            for _u in 0..(max_len1 - img1[i].len()){
                temp.push(' ');
            }
            let re = temp + img2[i].as_str();
            result.push(re);
        }else{
            let re = img1[i].clone() + img2[i].as_str();
            result.push(re);
        }
    }

    if img1.len() > img2.len(){
        result.extend_from_slice(&img1[min_len..img1.len()]);
    }else if img1.len() < img2.len(){
        for i in min_len..img2.len(){
            let mut temp = img2[i].clone();
            for _n in 0..max_len1{
                temp.insert(0,' ');
            }
            result.push(temp);
        }
    }
    result
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = [
    "<--",
    "#####",
    "<=="
    ].map(|v| v.to_string());

    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
    vcat(&data, &data), [
    "<--",
    "#####",
    "<==",
    "<--",
    "#####",
    "<=="
    ]);

    assert_eq!(
    hcat(&data, &data[..2]), [
    "<--  <--",
    "##########",
    "<=="
    ]);

    assert_eq!(
    hcat(&data[..2], &data), [
    "<--  <--",
    "##########",
    "     <=="
    ]);
}


