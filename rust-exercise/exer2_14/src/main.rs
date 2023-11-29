// fn main() {
//     println!("Hello");                 // => "Hello"
//     println!("Hello, {}!", "world");   // => "Hello, world!"
//     println!("The number is {}", 1);   // => "The number is 1"
//     println!("{:?}", (3, 4));          // => "(3, 4)"
//     println!("{value}", value=4);      // => "4"
//     println!("{} {}", 1, 2);           // => "1 2"
//     println!("{:04}", 42);             // => "0042" with leading zeros   
// }

fn main() {
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
}