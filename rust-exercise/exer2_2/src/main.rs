// CH 2.2.1

// 1. 移除某个部分让代码工作
// fn main() {
//     let x: i32 = 5;
//     let mut y  = 5;

//     y = x;
    
//     let z = 10; // 这里 z 的类型是? 
// }

// 2. 填空
// fn main() {
//     let v: u16 = 38_u8 as u16;
// }


// 3. 修改 `assert_eq!` 让代码工作
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }

// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// 4. 填空，让代码工作
// fn main() {
//     assert_eq!(i8::MAX, 127); 
//     assert_eq!(u8::MAX, 255); 
// }

// 5. 解决代码中的错误和 `panic`
// fn main() {
//     let v1 = 251_u16 + 8;
//     let v2 = u16::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
// }
 
// 6. 修改 `assert!` 让代码工作
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);
// }

// 7. 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64
// }

// 8. 使用两种方法来让下面代码工作
// fn main() {
//     assert!(((0.1_f64 + 0.2) - 0.3).abs() < 0.00001);
// }

// 9. 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 97..=122 {
//         println!("{}",c);
//     }
// }

// 10. 填空
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
// }

// 11. 填空，并解决错误
// fn main() {
//     // 整数加法
//     assert!(1u32 + 2 == 3);

//     // 整数减法
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);
    
//     assert!(3 * 50 == 150);

//     assert!(9 / 3 == 3);

//     assert!(24 % 5 == 4);
    
//     // 逻辑与或非操作
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // 位操作
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// CH 2.2.2

// 1. 修改2处 `assert_eq!` 让代码工作
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1), 4); 

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2), 4); 

//     println!("Success!")
// } 

// 2. 修改一行让代码正常打印
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// } 

// fn print_char(c : char) {
//     println!("{}", c);
// }

// 3. 使成功打印
// fn main() {
//     let f: bool = false;

//     let t = f;
//     if !t {
//         println!("Success!")
//     }
// } 

// 4.
// fn main() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(t, !f);

//     println!("Success!")
// }


// 5. 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let v = (2, 3);
//     assert_eq!(_v, implicitly_ret_unit());

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }

// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }

// 6. 单元类型占用的内存大小是多少？
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0); // unit type does't occupy any memeory space
//     println!("Success!");

//     let _ret_print = print!("");
//     assert!(_ret_print == ());
// }

// CH 2.2.3

// 1. 使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };
 
//     assert_eq!(v, 3);
// }

// 2.
// fn main() {
//     let v = {
//         let x = 3;
//         x
//     };

//     assert!(v == 3);
// }

// 3.
// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// CH 2.2.4

// 1.
// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }


// 2.
// fn main() {
//     print();
//  }
 
//  // 使用另一个类型来替代 i32
//  fn print() -> () {
//     println!("hello,world");
//  }
 
// 3. 用两种方法求解
// fn main() {
//     never_return();
// }

// fn never_return() -> ! {
//     // 实现这个函数，不要修改函数签名!
//     loop {
//     };
// }

// fn main() {
//     never_return();
// }

// fn never_return() -> ! {
//     panic!("no retun value");
// }

// 4. 发散函数( Diverging function )不会返回任何值，因此它们可以用于替代需要返回任何值的地方
// fn main() {
//     get_option(0);
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };
    
//     // 这里与其返回一个 None，不如使用发散函数替代
//     never_return_fn();
// }

// IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!()
// }

// IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     todo!();
// }

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     loop {
//         std::thread::sleep(std::time::Duration::from_secs(1))
//     }
// }

//5.
fn main() {
    // 填空
    let b = false;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
