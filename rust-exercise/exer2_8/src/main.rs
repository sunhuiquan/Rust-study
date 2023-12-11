// 2.8.1 a

// 1
// 填空
// struct A; // 具体的类型 `A`.
// struct S(A); // 具体的类型 `S`.
// struct SGen<T>(T); // 泛型 `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // 使用非泛型函数
//     reg_fn(S(A{})); // 具体的类型
//     gen_spec_t(SGen(A{})); // 隐式地指定类型参数  `A`.
//     gen_spec_i32(SGen(1)); // 隐式地指定类型参数`i32`.

//     // 显式地指定类型参数 `char`
//     generic::<char>(SGen('a'));

//     // 隐式地指定类型参数 `char`.
//     generic(SGen('a'));
// }

// 2
// 实现下面的泛型函数 sum
// fn sum<T>(lhs: T, rhs: T) -> T
// where
//     T: std::ops::Add<Output = T>,
// {
//     lhs + rhs
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }

// 3
// 实现一个结构体 Point 让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// 4
// 修改以下结构体让代码工作
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // 不要修改这行代码！
//     let p = Point{x: 5, y : "hello".to_string()};
// }

// 5
// 为 Val 增加泛型参数，不要修改 `main` 中的代码
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

// 6
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // 实现 mixup，不要修改其它代码！
//     fn mixup<V, W>(self, rhs: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: rhs.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point {
//         x: "Hello",
//         y: '中',
//     };

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }

// 7
// 修复错误，让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point { x: 5.0, y: 10.0 };
//     println!("{}", p.distance_from_origin())
// }

// 2.8.1 b
// 1
// 修复错误
// struct Array<T, const N: usize> {
//     data : [T; N]
// }

// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3]
//         }
//     ];
// }

// use std::usize;

// 2
// 填空
// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }

// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }

// 3
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }

// 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]); 
//     check_size([0i32; 191]);
//     check_size(["hello你好"; 47]); // size of &str ?
//     check_size([(); 31].map(|_| "hello你好".to_string()));  // size of String?
//     check_size(['中'; 191]); // size of char ?

//     println!("{}", core::mem::size_of::<&i32>()); // 8
//     println!("{}", core::mem::size_of::<&str>()); // 16
//     println!("{}", core::mem::size_of::<String>()); // 24
//     println!("{}", core::mem::size_of::<Vec<u8>>()); // 24
// }

// pub enum Assert<const CHECK: bool> {}

// pub trait IsTrue {}

// impl IsTrue for Assert<true> {}

