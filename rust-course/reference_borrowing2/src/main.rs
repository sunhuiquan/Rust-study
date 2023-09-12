fn main() {
    let mut s = String::from("test");
    let rs = &s;

    println!("print1: {}", s);
    println!("print2: {}", rs);

    func1(rs); // copy a unmutable reference
    println!("print3: {}", rs);

    func2(&mut s);
    // println!("print6: {}", rs); error

    func2(&mut s);

    let s2 = s; // ok, no one is borring s now

    let rs = &s2;
    func1(rs);
    println!("print4: {}", rs);

    // let _s3 = s2; error: cannot move out of `s2` because it is borrowed
    println!("print5: {}", rs);
}

fn func1(r: &String) {
    println!("printa: {}", r);
}

fn func2(r: &mut String) {
    println!("printb: {}", r);
}

// fn dangle() -> &String { // error: expected named lifetime parameter
//     let s = String::from("hello");
//     &s
// } 这里 s 离开作用域并被丢弃，其内存被释放，但是此时我们又尝试去返回它的引用，这意味着这个引用会指向一个无效的 String
// error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
