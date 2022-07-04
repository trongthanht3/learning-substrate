// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau
fn main() {
    for fibona in fibonacci_numbers() {
        println!("{}", fibona);
    }
}

// Khởi tạo ban đầu cho Fibonaci: 0, 1
fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}

#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.a.checked_add(self.b);
        if let Some(ref x) = n {
            std::mem::swap(&mut self.a, &mut self.b);
            self.a = x.clone();
        }
        n
        
    }
}

// // Bài 2: Lifetime
// // Yêu cầu: Sửa lỗi Lifetime 

// use std::fmt;
// struct StrDisplayable<'a>(Vec<&'a str>);

// impl<'a> fmt::Display for StrDisplayable<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for v in &self.0 {
//             write!(f, "\n{}", v)?;
//         }
//         Ok(())
//     }
    
// }

// fn main() {
//     let vec: Vec<&str> = vec!["a", "bc", "def"];
//     let vec_Foo = StrDisplayable(vec);
//     println!("{}", vec_Foo);
// }