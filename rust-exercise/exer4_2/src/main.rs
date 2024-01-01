// 4.2.1

// 4.2.2

/* 1. Refactoring the following code using iterators */
// fn main() {
//     let arr = [0; 10];
//     for item in &arr {
//         println!("{item}");
//     }
// }

/* 2. Fill in the blank */
// fn main() {
//     let mut v = Vec::new();
//     for n in 0..100 {
//         v.push(n);
//     }

//     assert_eq!(v.len(), 100);
// }

/* 3. Fill the blanks and fix the errors. Using two ways if possible */
// fn main() {
//     let mut v1 = vec![1, 2].into_iter();

//     assert_eq!(v1.next(), Some(1));
//     assert_eq!(v1.next(), Some(2));
//     assert_eq!(v1.next(), None);
// }

/* 4. Make it work */
// fn main() {
//     let arr = vec![0; 10];
//     for i in arr.iter() {
//         println!("{}", i);
//     }

//     println!("{:?}", arr);
// }

/* 5. Fill in the blank */
// fn main() {
//     let mut names = vec!["Bob", "Frank", "Ferris"];

//     for name in names.iter_mut() {
//         *name = match name {
//             &mut "Ferris" => "There is a rustacean among us!",
//             _ => "Hello",
//         }
//     }

//     println!("names: {:?}", names);
// }

/* 6. Fill in the blank */
// fn main() {
//     let mut values = vec![1, 2, 3];
//     let mut values_iter = values.iter_mut();

//     if let Some(v) = values_iter.next() {
//         *v = 0;
//     }

//     assert_eq!(values, vec![0, 2, 3]);
// }

//  7.
// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// // Implement `Iterator` for `Fibonacci`.
// // The `Iterator` trait only requires a method to be defined for the `next` element.
// impl Iterator for Fibonacci {
//     // We can refer to this type using Self::Item
//     type Item = u32;

//     /* Implement next method */
//     fn next(&mut self) -> Option<Self::Item> {
//         let res = self.next;
//         self.next += self.curr;
//         self.curr = res;
//         Some(res)
//     }
// }

// // Returns a Fibonacci sequence generator
// fn fibonacci() -> Fibonacci {
//     Fibonacci { curr: 0, next: 1 }
// }

// fn main() {
//     let mut fib = fibonacci();
//     assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(2));
//     assert_eq!(fib.next(), Some(3));
//     assert_eq!(fib.next(), Some(5));
// }

/* 8. Fill in the blank and fix the errors */
// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
//     let total: i32 = v1_iter.sum();

//     assert_eq!(total, 6);

//     println!("{:?}", v1);
// }

/* 9. Make it work */
// use std::collections::HashMap;
// fn main() {
//     let names = [("sunface", 18), ("sunfei", 18)];
//     let folks: HashMap<_, _> = names.into_iter().collect();

//     println!("{:?}", folks);

//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.into_iter().collect();

//     assert_eq!(v2, vec![1, 2, 3]);
// }

/* 10. Fill in the blanks */
// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

//     assert_eq!(v2, vec![2, 3, 4]);
// }

/* 11. Fill in the blanks */
// use std::collections::HashMap;
// fn main() {
//     let names = ["sunface", "sunfei"];
//     let ages = [18, 18];
//     let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

//     println!("{:?}", folks);
// }

/* 12. Fill in the blanks */
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
