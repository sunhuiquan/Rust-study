fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let _s2 = String::from("hello");

    let _s3 : &String;
    _s3 = &s1;
    // _s3 = &_s2; cannot assign twice to immutable variable

    let mut str = String::from("hello");

    {
        let _r1 = &mut str;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let _r2 = &mut str;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn change(s: &String) {
//     s.push_str(", world");
// }
