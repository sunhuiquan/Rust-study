// ch 2.9.1

// 1
// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v = Vec::from(arr);
//     is_vec(&v);

//     let v = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
//     let v = vec!(1, 2, 3);
//     is_vec(&v);
    
//     // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
//     // 使用 Vec::new 和 `for` 来重写下面这段代码
//     // let v1 = vec!(arr);
//     let mut v1 = Vec::new();
//     for e in &v {
//         v1.push(*e);
//     }
//     is_vec(&v1);
 
//     assert_eq!(v, v1);

//     println!("Success!")
// }

// fn is_vec(v: &Vec<u8>) {}

// 2
// 填空
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
    
//     let mut v2 = Vec::new();
//     v2.extend([1, 2, 3]);

//     assert_eq!(v1, v2);

//     println!("Success!")
// }

// 3

// 填空
// fn main() {
//     // array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
 
//     assert_eq!(v1, v2);
 
    
//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     // 迭代器 Iterators 可以通过 collect 变成 Vec
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!")
// }

// 4
// 修复错误并实现缺失的代码
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..3 {
//         println!("{:?}", v[i])
//     }

//     for i in 0..5 {
//         match v.get(i) {
//             Some(_) => {
//                 v[i] += 1;
//             },
//             None => {
//                 v.push(i + 2);
//             }
//         }
//     }
    
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!")
// }

// 5
// 修复错误
// fn main() {
//     let mut v = vec![1, 2, 3];

//     let slice1 = &v[..];
//     // 越界访问将导致 panic.
//     // 修改时必须使用 `v.len`
//     let slice2 = &v[0..v.len()];

//     assert_eq!(slice1, slice2);
    
//     // 切片是只读的
//     // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
//     let vec_ref = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..];
//     // slice3.push(4);

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!")
// }

// 6
// 修复错误
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     // ...但是下面的代码会造成新的内存分配
//     vec.push(11); // 超过当前容量，重新分配
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);


//     // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
    
//     println!("Success!")
// }

// 7
// #[derive(Debug, PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // 填空
//     let v : Vec<IpAddr>= vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    
//     // 枚举的比较需要派生 PartialEq 特征
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success!")
// }

// 8
// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }

// fn main() {
//     // 填空
//     let v: Vec<Box<dyn IpAddr>>= vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

// ch 2.9.2

// 1
// 填空并修复错误
// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);

//     // get 返回一个 Option<&V> 枚举值
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
//         // 索引返回一个值 V
//         let score = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score)
//     }
// }

// 2
// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }

//     // 使用两种方法实现 team_map2
//     // 提示:其中一种方法是使用 `collect` 方法
//     // let teams_map2 = teams.into_iter().collect();
//     let teams_map2 = HashMap::from(teams);

//     assert_eq!(teams_map1, teams_map2);

//     println!("Success!")
// }

// 3
// 填空
// use std::collections::HashMap;
// fn main() {
//     // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
//     let mut player_stats = HashMap::new();

//     // 查询指定的 key, 若不存在时，则插入新的 kv 值
//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     // 通过函数来返回新的值
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100);

//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);

//     println!("Success!")
// }

// fn random_stat_buff() -> u8 {
//     // 为了简单，我们没有使用随机，而是返回一个固定的值
//     42
// }

// 4
// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
// use std::collections::HashMap;

// #[derive(Debug, PartialEq, Eq, Hash)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     // 使用 HashMap 来存储 viking 的生命值
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);

//     // 使用 derive 的方式来打印 viking 的当前状态
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }

// 5
// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
use std::collections::HashMap;
fn main() {
  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello";
  let mut m2 = HashMap::new();
  // 所有权在这里发生了转移
  m2.insert(v2, v1);

  assert_eq!(v2, "hello");

   println!("Success!")
}
