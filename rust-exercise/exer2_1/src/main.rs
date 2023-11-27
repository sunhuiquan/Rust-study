// 2.1
// 修复下面代码的错误并尽可能少的修改

// fn main() {
//     let x: i32 = 1; // 未初始化，但被使用
//     let _y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x); 
// }

// 2.2 可以使用 mut 将变量标记为可变
// 完形填空，让代码编译

// fn main() {
//     let mut x = 1;
//     x += 2; 

//     println!("x = {}", x); 
// }

// 2.3 作用域是一个变量在程序中能够保持合法的范围
// 修复下面代码的错误并使用尽可能少的改变

// fn main() {
//     let x: i32 = 10;
//     let y;
//     {
//         y = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}, y 的值是 {}", x, y); 
// }

// 2.4
// 修复错误

// fn main() {
//     let x = define_x();
//     println!("{}, world", x); 
// }

// fn define_x() -> &'static str {
//     let x: &str = "hello";
//     x
// }

// 2.5 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )
// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)

// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// 2.6 修改一行代码以通过编译

// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let mut x = x; 
//     x += 3;


//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!"; 
// }

// 2.7 使用以下方法来修复编译器输出的 warning :

// fn main() {
//     let _x = 1; 
// }

// #[allow(unused_variables)]
// fn main() {
//     let x = 1; 
// }

// 2.8  我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
// 提示: 可以使用变量遮蔽或可变性

// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (x, y) = (1, 2);
//     let mut x = x;
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// 2.9

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3, 2]);
} 
