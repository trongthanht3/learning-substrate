// // Exercise 1
// // Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
// fn main() {
    
//     let x = change_value(10,&mut 20);
// }



// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output =3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }


//Exercise 2
// // Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// // Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     print!("lim: {}", is_prime(61));

//     while count < 10 {
//         num += 2;
//         println!("{}", num);
//         // if vector_is_prime(num, &primes) {
//         //     count += 1;
//         //     if is_prime(num) {
//         //         primes.push(num);
//         //     }
//         // }
//         if is_prime(num) {
//             primes.push(num);
//             count += 1;
//         }
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
//     for i in p {
//         if num > *i && num % i != 0 {
//             return false;
//         }
//     }

//     true
// }

// fn is_prime(num: u64) -> bool {
//     if num == 2 {
//         return true;
//     }
//     if num == 3 {
//         return true;
//     }

//     let lim = (num as f64).sqrt() as u64;
//     for i in 2..lim {
//         if num % i == 0 {
//             return false;
//         }
//     }

//     return true;
// }


// //Exercise 3
// // Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;
    
//     //for n in &mut values {
//     for n in v.into_iter() {
//         max = std::cmp::max(max, *n);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
//     //for n in &mut values {
//     for n in v.into_iter() {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }


// //Exercise 4
// // Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// // Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
// fn main(){
//     let mut a = vec![1,2,3,4,5];
//     let mut i = 0;
//     let c = 0;

//     let (a, c) = test(&mut a);
//     println!("{}",c);
//     println!("{:?}",a);

// }

// pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {
//     let mut b:Vec<u8>  = Vec::new();
//     let mut c:u8 = 0;
//     loop {
//         // if a.len() == 0 { break; }
//         // let d = a.pop().unwrap();
//         // c = c+d;
//         // b.push(d);
//         if a.len() == 0 {
//             break;
//         }
//         let d = a.pop().unwrap();
//         c = c+d;
//         b.push(d);
//     }
//     (b, c as i32)
// }