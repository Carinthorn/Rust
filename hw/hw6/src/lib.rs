//q1.1
fn min_max_avg(nlist: &[i32]) -> (i32, i32 , f32){
    if nlist.len() == 0{
        return (0, 0, 0.0);
    }
    let mut min: i32 = nlist[0];
    let mut max: i32 = nlist[0];
    let mut avg: i32 = 0;
    
    for i in nlist{
        if i < &min{
            min = *i;
        }else if i > &max{
            max = *i;
        }
        avg += i;
    }
    return (min, max, (avg as f32/nlist.len() as f32));
}

#[test]
fn check_mma(){
    assert_eq!(min_max_avg(&[]), (0, 0, 0.0));
    assert_eq!(min_max_avg(&[0]), (0, 0, 0.0));
    assert_eq!(min_max_avg(&[42]), (42, 42, 42.0));
    assert_eq!(min_max_avg(&[2, 11, 3, 4, 7]), (2, 11, 5.4));
    assert_eq!(min_max_avg(&[-2, -11, -3, -4, -7]), (-11, -2, -5.4));
}

//q1.2
fn cal_partial_sums(input: &[i32]) -> Vec<i32>{
    if input.len() == 0{
        return vec![];
    }

    let mut result: Vec<i32> = vec![input[0]];
    let mut num: i32 = input[0];
    for i in 1..input.len(){
        num += input[i];
        result.push(num);
    }
    result
}

#[test]
fn test_check_mma(){
    assert_eq!(cal_partial_sums(&[]), []);
    assert_eq!(cal_partial_sums(&[0]), [0]);
    assert_eq!(cal_partial_sums(&[5]), [5]);
    assert_eq!(cal_partial_sums(&[-1, -2, -3, -4, -5]), [-1, -3, -6, -10, -15]);
    assert_eq!(cal_partial_sums(&[2, 11, 3, 4, 7]), [2, 13, 16, 20, 27]);
}

//q2.1
fn pack_number_tuples3(list1: &[i32], list2: &[i32], list3: &[i32]) -> Vec<(i32, i32, i32)>{
    let mut temp = vec![list1.len(), list2.len(), list3.len()];
    let mut result = Vec::new();
    temp.sort();

    let len = temp[temp.len() - 1];
    for i in 0..len{
        let l1 = if list1.len() < len && i == len - 1 { 0 }else{ list1[i] };
        let l2 = if list2.len() < len && i == len - 1 { 0 }else{ list2[i] };
        let l3 = if list3.len() < len && i == len - 1 { 0 }else{ list3[i] };       
        result.push((l1, l2, l3));
    }
    result
}

#[test]
fn test_pack_number_tuples3(){
    assert_eq!(pack_number_tuples3(&[], &[], &[]), []);
    assert_eq!(pack_number_tuples3(&[1], &[2], &[3]), [(1, 2, 3)]);
    assert_eq!(pack_number_tuples3(&[1], &[2, 4], &[3]), [(1, 2, 3), (0, 4, 0)]);
    assert_eq!(pack_number_tuples3(&[1, 2], &[4, 3], &[5]), [(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]), [(1, 4, 7), (2, 5, 8), (3, 6, 9)]);
}

//q2.2
fn pack_number_tuples_s3(list1:&[i32], list2:&[i32], list3:&[i32]) -> Vec<(i32, i32, i32)>{
    let n = list1.len().min(list2.len()).min(list3.len());
    let mut result: Vec<(i32, i32, i32)> = Vec::new();
    for i in 0..n{
        result.push((list1[i], list2[i], list3[i]));
    }
    result
}

#[test]
fn test_pack_number_tuples_s3(){
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), []);
    assert_eq!(pack_number_tuples_s3(&[1], &[2], &[3]), [(1, 2, 3)]);
    assert_eq!(pack_number_tuples_s3(&[1, 2, 3], &[4, 5], &[6]), [(1, 4, 6)]);
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[5]),[(1, 4, 5)]);
    assert_eq!(pack_number_tuples_s3(&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]),[(1, 4, 7), (2, 5, 8), (3, 6, 9)]);
}

//q3.1
fn unpack_number_tuples(inp: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>){
    let mut result1: Vec<i32> = Vec::new();
    let mut result2: Vec<i32> = Vec::new();

    for i in inp{
        result1.push(i.0);
        result2.push(i.1);
    }
    let mut result: (Vec<i32>, Vec<i32>) = (result1.clone(), result2.clone());
    result
}

#[test]
fn test_unpack_number_tuples(){
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 2)]), (vec![1], vec![2]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (2, 5), (3, 6)]), (vec![1, 2, 3], vec![4, 5, 6]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]),  (vec![1, 3, 2], vec![4, 2, 1]));
    assert_eq!(unpack_number_tuples(&[(-1, -4), (-2, -5), (-3, -6)]), (vec![-1, -2, -3], vec![-4, -5, -6]));
}

//q3.2
fn unpack_number_tuples3(inp: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut result1: Vec<i32> = Vec::new();
    let mut result2: Vec<i32> = Vec::new();
    let mut result3: Vec<i32> = Vec::new();

    for i in inp{
        result1.push(i.0);
        result2.push(i.1);
        result3.push(i.2);
    }
    let mut result: (Vec<i32>, Vec<i32>,  Vec<i32>) = (result1.clone(), result2.clone(), result3.clone());
    result
}

#[test]
fn test_unpack_number_tuples3(){
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(1, 2, 3)]), (vec![1], vec![2], vec![3]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1)]), (vec![1, 2], vec![4, 2], vec![5, 1]));
    assert_eq!(unpack_number_tuples3(&[(-1, -4, -5), (-2, -2, -1)]), (vec![-1, -2], vec![-4, -2], vec![-5, -1]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1), (4, 5, 6)]), (vec![1, 2, 4], vec![4, 2, 5], vec![5, 1, 6]));
}

