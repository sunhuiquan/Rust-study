// 2.4.1 a

// 1
// 修复错误，不要新增代码行
// fn main() {
//     let s: &str = "hello, world";
// }

// 2
// 使用至少两种方法来修复错误
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

// 3
// 填空
// fn main() {
//     let mut s = String::new();
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");
// }

// 4
// 修复所有错误，并且不要新增代码行
// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s)
// }

// 5
// 填空
// fn main() {
//     let s = String::from("I like dogs");
//     // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "I like cats")
// }

//6
// 修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = "world!";
//     let s3 = s1.clone() + s2;
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1);
// }

// 7
// 使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world";
//     greetings(s.to_string());
// }

// fn greetings(s: String) {
//     println!("{}",s)
// }

// 8
// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = &s;
// }

// 9
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );

//     // 还能使用 \ 来连接多行字符串
//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }

// 10
/* 填空并修复所有错误 */
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // 修改上面的行让代码工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);

//     // 填空
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }

// 11
// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1];
//     // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//     assert_eq!(h, "h");

//     let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//     assert_eq!(h1, "中");
// }

// 12
// fn main() {
//     // 填空，打印出 "你好，世界" 中的每一个字符
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }

// 2.4.1 b

// 1
// 修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1 = &arr[0..2];

//     let s2: &str = "hello, world";
// }

// 2
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];

//     let slice = &arr[..2];

//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 16);
// }

// 3
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//    // 填空让代码工作起来
//    let slice = &arr[1..4];
//    assert_eq!(slice, &[2, 3, 4]);
//  }

// 4
// fn main() {
//     let s = String::from("hello");

//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[..2];

//     assert_eq!(slice1, slice2);
// }

// 5
// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..3];

//     assert!(slice == "你");
// }

// 6
// 修复所有错误
// fn main() {
//     let mut s = String::from("hello world");

//     // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
//     // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
//     let ch = first_character(&s);

//     println!("the first character is: {}", ch);

//     s.clear(); // error!
// }
// fn first_character(s: &str) -> &str {
//     &s[..1]
// }

// 2.4.1 c

// 1
// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
// fn main() {
//     let mut s: String = String::from("hello, ");
//     s.push_str("world");
//     s.push('!');

//     move_ownership(s.clone());

//     assert_eq!(s, "hello, world!");

//     println!("Success!")
// }

// fn move_ownership(s: String) {
//     println!("ownership of \"{}\" is moved here!", s)
// }

// 2
// 填空
// fn main() {
//     let mut s = String::from("hello, world");

//     let slice1: &str = &s; // 使用两种方法
//     assert_eq!(slice1, "hello, world");

//     let slice2 = &s[..5];
//     assert_eq!(slice2, "hello");

//     let slice3: &mut String = &mut s;
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");

//     println!("Success!")
//  }

// 3
// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答: 2
// fn main() {
//     // 基于 `&str` 类型创建一个 String,
//     // 字符串字面量的类型是 `&str`
//    let s: String = String::from("hello, world!");

//    // 创建一个切片引用指向 String `s`
//    let slice: &str = &s;

//    // 基于刚创建的切片来创建一个 String
//    let s: String = slice.to_string();

//    assert_eq!(s, "hello, world!");

//    println!("Success!")
// }

// 4
// 填空并修复错误
// fn main() {
//     let s = String::from("hello, 世界");
//     let slice1 = &s[..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
//     assert_eq!(slice1, "h");

//     let slice2 = &s[7..10];// 提示: `世` 在 UTF-8 编码中占用 3 个字节
//     assert_eq!(slice2, "世");

//     // 迭代 s 中的所有字符
//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }

//     println!("Success!")
// }

// 5
// 填空
// fn main() {
//     let mut s = String::from("hello");

//     let v = vec![104, 101, 108, 108, 111];

//     // 将字节数组转换成 String
//     let s1 = String::from_utf8(v);

//     assert_eq!(s, s1.unwrap());

//     println!("Success!")
// }

// 6
// 修改下面的代码以打印如下内容:
// 25
// 25
// 25
// 循环中不会发生任何内存分配
// fn main() {
//     let mut s = String::with_capacity(25);

//     println!("{}", s.capacity());

//     for _ in 0..2 {
//         s.push_str("hello");
//         println!("{}", s.capacity());
//     }

//     println!("Success!")
// }

// 7
// 填空
// use std::mem;

// fn main() {
//     let story = String::from("Rust By Practice");

//     // 阻止 String 的数据被自动 drop
//     let mut story = mem::ManuallyDrop::new(story);

//     let ptr = story.as_mut_ptr();
//     let len = story.len();
//     let capacity = story.capacity();

//     assert_eq!(16, len);

//     // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
//     // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
//     let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

//     assert_eq!(*story, s);

//     println!("Success!")
// }

// 2.4.2

// 1
// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // 元组的成员还可以是一个元组
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     // 填空让代码工作
//     let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }

// 2
// 修改合适的地方，让代码工作
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface");
// }

// 3
// 修复代码错误
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
//     println!("too long tuple: {:?}", too_long_tuple);
// }

// 4
// fn main() {
//     let tup = (1, 6.4, "hello");

//     // 填空
//     let (x, z, y) = tup;

//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }

// 5
// fn main() {
//     let (x, y, z);

//     // 填空
//     (y, z, x) = (1, 2, 3);

//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }

// 6
// fn main() {
//     // 填空，需要稍微计算下
//     let (x, y) = sum_multiply((2, 3));

//     assert_eq!(x, 5);
//     assert_eq!(y, 6);
// }

// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }

// 2.4.5

// 1
// fn main() {
//     // 使用合适的类型填空
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];

//     // 修改以下代码，让它顺利运行
//     assert!(arr.len() == 5);
// }

// 2
// fn main() {
//     // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];
//     let arr2 = &arr;
//     let arr3 = &arr2;
//     let arr4 = &arr[..];

//     // 填空
//     // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//     // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//     assert!(std::mem::size_of_val(&arr) == 12);
//     assert!(std::mem::size_of_val(arr2) == 12);
//     assert!(std::mem::size_of_val(&arr2) == 8);
//     assert!(std::mem::size_of_val(arr4) == 12);
//     assert!(std::mem::size_of_val(&arr4) == 16);
// }

// 3
// fn main() {
//     // 填空
//     let list: [i32; 100] = [1; 100];

//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
// }

// 4
// fn main() {
//     // 修复错误
//     let _arr = [1, 2, 3];
// }

// 5
// fn main() {
//     let arr = ['a', 'b', 'c'];

//     let ele = arr[0]; // 只修改此行来让代码工作

//     assert!(ele == 'a');
// }

// 6
// 修复代码中的错误
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];

//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let name0 = names.get(0).unwrap();

//     // 但是下标索引就存在越界的风险了
//     let _name1 = &names[1];
// }
