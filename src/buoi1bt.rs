use std::string;
use rand::Rng;
use std::io::stdin;

fn main() {    
    let mut arr: &[i32] = &[1,54,3,1,2,5,6,7,7,3,2];
    let sub_arr: &[i32] = &[1,54];

    println!("{} ", sub_arr.len());
    println!("Result {}", check_sub_array(arr, sub_arr));
}

fn check_sub_array(arr: &[i32], sub_arr: &[i32]) -> bool {
    println!("{:?}", arr);

    let mut check_arr: bool = false;

    for i in 0..(arr.len() - sub_arr.len() + 1) {
        println!("-----------------------");
        for k in 0..sub_arr.len() {
            print!("{}-{} | ", arr[i+k], sub_arr[k]);
            if sub_arr[k] != arr[i+k] {
                break;
            }
            if k == sub_arr.len()-1 {
                check_arr = true;
            }
        }
    }

    return check_arr;
}


fn find_substring() {
    let sample_str: String = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");
    let v: Vec<_> = sample_str.match_indices("This is").collect();
    println!("{:?}", v);
}