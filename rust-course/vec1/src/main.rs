fn main() {
    let _v1: Vec<i32> = Vec::new();
    
    let mut v2 = Vec::new();
    v2.push(1);

    let _v3: Vec<i32> = Vec::with_capacity(10);

    let _v4 = vec![1, 2, 3];

    println!("Hello, world!");

    let mut a = 1;
    let r = &mut a;
    *r = 2;
    println!("Hello, world! {}", r);
}
