// 2.10

// 1
/* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */

// `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
// 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
// fn main() {
//     let i = 3; // Lifetime for `i` starts. ────────────────┐
//     //                                                     │
//     { //                                                   │
//         let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
//         //                                                ││
//         println!("borrow1: {}", borrow1); //              ││
//     } // `borrow1 ends. ──────────────────────────────────┘│
//     //                                                     │
//     //                                                     │
//     { //                                                   │
//         let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
//         //                                                ││
//         println!("borrow2: {}", borrow2); //              ││
//     } // `borrow2` ends. ─────────────────────────────────┘│
//     //                                                     │
// }   // Lifetime ends. ─────────────────────────────────────┘

// 2
// #![allow(unused)]
// fn main() {
// {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }                         // ----------+
// }

/* 像上面的示例一样，为 `r` 和 `x` 标准生命周期，然后从生命周期的角度. */
// We can't borrow a item whose lifetime is smaller.
// fn main() {
//     {
//         let r;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
// }

// 3
/* 添加合适的生命周期标注，让下面的代码工作 */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}

// 4
/* 使用三种方法修复下面的错误  */
// fn invalid_output() -> String {
//     String::from("foo")
// }

// fn invalid_output() -> &'static str {
//     "foo"
// }

// fn invalid_output(s: &String) -> &String {
//     s
// }

// fn main() {}

// 5
// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// /* 让下面的代码工作 */
// fn failed_borrow() {
//     let _x = 12;

//     // ERROR: `_x` 活得不够久does not live long enough
//     let y: &i32 = &_x;

//     // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
//     // 你不能将一个小的生命周期强转成大的
// }

// fn main() {
//     let (four, nine) = (4, 9);

//     print_refs(&four, &nine);
//     // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

//     failed_borrow();
//     // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
// }

// 6
/* 增加合适的生命周期标准，让代码工作 */
// `i32` 的引用必须比 `Borrowed` 活得更久
// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
// #[derive(Debug)]
// struct NamedBorrowed<'a, 'b> {
//     x: &'a i32,
//     y: &'b i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x = 18;
//     let y = 15;

//     let single = Borrowed(&x);
//     let double = NamedBorrowed { x: &x, y: &y };
//     let reference = Either::Ref(&x);
//     let number = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);

//     {}
// }

// 7
/* 让代码工作 */
// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// fn main() {
//     let var_a = 35;

//     {
//         let example: Example;
//         let var_b = NoCopyType {};

//         /* 修复错误 */
//         example = Example {
//             a: &var_a,
//             b: &var_b,
//         };
//         println!("(Success!) {:?}", example);
//     }
// }

// 8
// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// /* 修复函数的签名 */
// fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType {
//     foo.b
// }

// fn main() {
//     let no_copy = NoCopyType {};
//     let example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success!")
// }

// 9
// /* 添加合适的生命周期让下面代码工作 */
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// fn main() {}

// 10
/* 移除所有可以消除的生命周期标注 */
fn nput(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 {
    x
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }

    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}
