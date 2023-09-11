fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里，所以后面 s 不再有效
    // println!("{}", s);           // compile error

    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 拷贝到函数里，所以在后面可继续使用 x
    println!("{}", x);

    let _s1 = gives_ownership();         // gives_ownership 将返回值移动给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let _s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中，然后该函数也将返回值移给 s3
}
// 同 C++ 的释放顺序一样，栈上创建的变量后进先出，所以后面创建的对象先要离开作用域释放
// 这里, s3 首先移出作用域并丢弃释放内存；然后 s2 也移出作用域，但已被移走，所以什么也不会发生；然后 s1 移出作用域并被丢弃；然后 x 移出了作用域并丢弃；最后是 s，但 s 的已被移动走，所以不会再释放

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {              // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello");  // some_string 进入作用域.
    some_string                               // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // takes_and_gives_back 将传入字符串并返回该值
    a_string                                          // 返回 a_string 并移出给调用的函数
}
