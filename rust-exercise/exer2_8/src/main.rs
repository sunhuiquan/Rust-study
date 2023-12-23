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

// 2.8.2

// 1
// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }

// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }

//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }

// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!")
// }

// 2
// `Centimeters`, 一个元组结构体，可以被比较大小
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// // `Inches`, 一个元组结构体可以被打印
// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// 添加一些属性让代码工作
// 不要修改其它代码！
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);

// fn main() {
//     let _one_second = Seconds(1);

//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_true = _one_second > _one_second;

//     let foot = Inches(12);

//     println!("One foot equals {:?}", foot);

//     let meter = Centimeters(100.0);

//     let cmp = if foot.to_centimeters() < meter {
//         "smaller"
//     } else {
//         "bigger"
//     };

//     println!("One foot is {} than one meter.", cmp);
// }

// 3
// use std::ops::Mul;

// // 实现 fn multiply 方法
// // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
// fn multiply<T: Mul<Output = T>>(lhs: T, rhs: T) -> T {
//     lhs * rhs
// }

// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));

//     println!("Success!")
// }

// 4
// 修复错误，不要修改 `main` 中的代码!
// use std::ops;

// struct Foo;
// struct Bar;

// #[derive(Debug, PartialEq)]
// struct FooBar;

// #[derive(Debug, PartialEq)]
// struct BarFoo;

// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }

// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Bar - Foo, BarFoo);

//     println!("Success!")
// }

// 5
// 实现 `fn summary`
// 修复错误且不要移除任何代码行
// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// 在下面实现 `fn summary` 函数
// fn summary(s: &impl Summary) {}

// 6
// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Sheep {}
//     }
// }

// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!(
//         "You've randomly chosen an animal, and it says {}",
//         animal.noise()
//     );
// }

// 7
// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }

// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T>(x: T, y: T) -> T {
//     x + y
// }

// 8
// 修复代码中的错误
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }

// #[derive(Debug, PartialEq, PartialOrd)]
// struct Unit(i32);

// fn main() {
//     let pair = Pair {
//         x: Unit(1),
//         y: Unit(3),
//     };

//     pair.cmp_display();
// }

// 9
// 填空
// fn example1() {
//     // `T: Trait` 是最常使用的方式
//     // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
//     struct Cacher<T: Fn(u32) -> u32> {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 }
//             }
//         }
//     }

//     let mut cacher = Cacher::new(|x| x + 1);
//     assert_eq!(cacher.value(10), 11);
//     assert_eq!(cacher.value(15), 11);
// }

// fn example2() {
//     // 还可以使用 `where` 来约束 T
//     struct Cacher<T>
//     where
//         T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T> Cacher<T>
//     where
//         T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 }
//             }
//         }
//     }

//     let mut cacher = Cacher::new(|x| x + 1);
//     assert_eq!(cacher.value(20), 21);
//     assert_eq!(cacher.value(25), 21);
// }

// fn main() {
//     example1();
//     example2();

//     println!("Success!")
// }

// 2.8.3

// 1
// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }

// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swan swan".to_string()
//     }
// }

// fn main() {
//     // 填空
//     let duck = Duck {};
//     duck.swim();

//     let bird = hatch_a_bird(2);
//     // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
//     // bird.swim();
//     // 但它依然可以叫唤
//     assert_eq!(bird.quack(), "duck duck");

//     let bird = hatch_a_bird(1);
//     // 这只鸟儿忘了如何飞翔，因此以下代码会报错
//     // bird.fly();
//     // 但它也可以叫唤
//     assert_eq!(bird.quack(), "swan swan");

//     println!("Success!")
// }

// // 实现以下函数
// fn hatch_a_bird(kind: i32) -> Box<dyn Bird> {
//     if kind == 2 {
//         Box::new(Duck {})
//     } else {
//         Box::new(Swan {})
//     }
// }

// 2
// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     // 填空
//     let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

//     for bird in birds {
//         bird.quack();
//         // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
//         // 因此，以下代码会报错
//         // bird.fly();
//     }
// }

// 3
// 填空
// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn main() {
//     let x = 1.1f64;
//     let y = 8u8;

//     // draw x
//     draw_with_box(Box::new(x));

//     // draw y
//     draw_with_ref(&y);

//     println!("Success!")
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw_with_ref(x: & dyn Draw) {
//     x.draw();
// }

// 4
// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Foo for String {
//     fn method(&self) -> String {
//         format!("string: {}", *self)
//     }
// }

// // 通过泛型实现以下函数
// fn static_dispatch<T: Foo>(v: T) {
//     v.method();
// }

// // 通过特征对象实现以下函数
// fn dynamic_dispatch(v: &dyn Foo) {
//     v.method();
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!")
// }

// 5
// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> u32 { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> String { self.clone() }
// }

// fn my_function(x: impl MyTrait) -> impl MyTrait  {
//     x.f()
// }

// fn main() {
//     my_function(13_u32);
//     my_function(String::from("abc"));
// }

// trait MyTrait {
//     fn f(&self) -> Box<dyn MyTrait>;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Box<dyn MyTrait> {
//         Box::new(42)
//     }
// }

// impl MyTrait for String {
//     fn f(&self) -> Box<dyn MyTrait> {
//         Box::new(self.clone())
//     }
// }

// fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
//     x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("Success!")
// }

// 2.8.4

// 1
// struct Container(i32, i32);

// // 使用关联类型实现重新实现以下特征
// // trait Contains {
// //    type A;
// //    type B;

// // trait Contains<A, B> {
// //     fn contains(&self, _: &A, _: &B) -> bool;
// //     fn first(&self) -> i32;
// //     fn last(&self) -> i32;
// // }

// trait Contains {
//     type A;
//     type B;

//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// // impl Contains<i32, i32> for Container {
// //     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
// //         (&self.0 == number_1) && (&self.1 == number_2)
// //     }
// //     // Grab the first number.
// //     fn first(&self) -> i32 { self.0 }

// //     // Grab the last number.
// //     fn last(&self) -> i32 { self.1 }
// // }

// impl Contains for Container {
//     type A = i32;
//     type B = i32;

//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }
//     // Grab the first number.
//     fn first(&self) -> i32 {
//         self.0
//     }

//     // Grab the last number.
//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// // fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
// //     container.last() - container.first()
// // }

// fn difference<C: Contains>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;

//     let container = Container(number_1, number_2);

//     println!(
//         "Does container contain {} and {}: {}",
//         &number_1,
//         &number_2,
//         container.contains(&number_1, &number_2)
//     );
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());

//     println!("The difference is: {}", difference(&container));
// }

// 2
// use std::{fmt::Debug, ops::Sub};

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Sub<Point<T>> for Point<T>
// where
//     T: Sub<T, Output = T>,
// {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("Success!")
// }

// 3
// trait Pilot {
//     fn fly(&self) -> String;
// }

// trait Wizard {
//     fn fly(&self) -> String;
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) -> String {
//         String::from("This is your captain speaking.")
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) -> String {
//         String::from("Up!")
//     }
// }

// impl Human {
//     fn fly(&self) -> String {
//         String::from("*waving arms furiously*")
//     }
// }

// fn main() {
//     let person = Human;

//     assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
//     assert_eq!(Wizard::fly(&person), "Up!");

//     assert_eq!(person.fly(), "*waving arms furiously*");

//     println!("Success!")
// }

// 4
// trait Person {
//     fn name(&self) -> String;
// }

// // Person 是 Student 的 supertrait .
// // 实现 Student 需要同时实现 Person.
// trait Student: Person {
//     fn university(&self) -> String;
// }

// trait Programmer {
//     fn fav_language(&self) -> String;
// }

// // CompSciStudent (computer science student) 是 Programmer
// // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
// trait CompSciStudent: Programmer + Student {
//     fn git_username(&self) -> String;
// }

// fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
//     format!(
//         "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
//         student.name(),
//         student.university(),
//         student.fav_language(),
//         student.git_username()
//     )
// }

// struct CSStudent {
//     name: String,
//     university: String,
//     fav_language: String,
//     git_username: String
// }

// // 为 CSStudent 实现所需的特征
// impl Programmer for CSStudent {
//     fn fav_language(&self) -> String {
//         String::from("rust")
//     }
// }

// impl Person for CSStudent {
//     fn name(&self) -> String {
//         String::from("jerk")
//     }
// }

// impl Student for CSStudent {
//     fn university(&self) -> String {
//         String::from("mit")
//     }
// }

// impl CompSciStudent for CSStudent {
//     fn git_username(&self) -> String {
//         String::from("jerk_user")
//     }
// }

// fn main() {
//     let student = CSStudent {
//         name: "Sunfei".to_string(),
//         university: "XXX".to_string(),
//         fav_language: "Rust".to_string(),
//         git_username: "sunface".to_string()
//     };

//     // 填空
//     println!("{}", comp_sci_student_greeting(&student));
// }

// 5
use std::fmt;

// 定义一个 newtype `Pretty`
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
