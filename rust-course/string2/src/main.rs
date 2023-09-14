fn main() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    
    let result = string_append + &string_rust; // &string_rust会自动解引用为&str
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("Concat result: {}", result);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
    println!("{}",s1);
    println!("{}",s2);
}