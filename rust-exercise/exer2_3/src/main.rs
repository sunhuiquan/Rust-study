// ch 2.3.1

// 1
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = &x;
//     println!("{},{}",x,y);
// }

// 2
// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// // 只能修改下面的代码!
// fn take_ownership(s: String) -> String{
//     println!("{}", s);
//     s
// }

// 3
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     let _s = s.as_bytes();
//     s
// }

// 4
// 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");

//     print_str(s.clone());

//     println!("{}", s);
// }

// fn print_str(s: String)  {
//     println!("{}",s)
// }

// 5
// 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }

// 6
// fn main() {
//     let s = String::from("hello, ");
    
//     // 只修改下面这行代码 !
//     let mut s1 = s;

//     s1.push_str("world")
// }

// 7
// fn main() {
//     let x = Box::new(5);
    
//     let mut y = x.clone();      // 完成该行代码，不要修改其它行！
    
//     *y = 4;
    
//     assert_eq!(*x, 5);
// }

// 8
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//     let _s = t.0;
 
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
// }
 
// 9
// fn main() {
//     let t = (String::from("hello"), String::from("world"));
 
//     // 填空，不要修改其它代码
//     let (s1, s2) = (&t.0, &t.1);
 
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//  }

// ch 2.3.2

// 1
// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = &x;
 
//     println!("x 的内存地址是 {:p}", p);
// }

// 2
// fn main() {
//     let x = 5;
//     let y = &x;

//     // 只能修改以下行
//     assert_eq!(5, *y);
// }

// 3
// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s)
// }

// fn borrow_object(s: &String) {}

// 4
// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     push_str(&mut s)
// }

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// 5
// fn main() {
//     let mut s = String::from("hello, ");

//     // 填写空白处，让代码工作
//     let p = &mut s;
    
//     p.push_str("world");
// }

// 6
// fn main() {
//     let c = '中';

//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;

//     assert_eq!(*r1, *r2);
    
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1),get_addr(r2));
// }

// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// 7
// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
// fn main() {
//     let s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);
// }

// 8
// fn main() {
//     // 通过修改下面一行代码来修复错误
//     let mut s = String::from("hello, ");

//     borrow_object(&mut s)
// }

// fn borrow_object(s: &mut String) {}

// 9
// 下面的代码没有任何错误
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s);
    
//     s.push_str("world");
// }

// fn borrow_object(s: &String) {}

// 10
// 注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
    
//     // println!("{}",r1);
// }

// 11
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    r1.push('a');
}
