struct Struct {
    e: i32
}

fn r() -> u8 {
    return 150
}

fn main() {
    let (a1, mut b1) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a1 = {:?}, b1 = {:?}", a1, b1);

    b1 = true;
    assert_eq!(a1, b1);

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    const MAX_POINTS: u32 = 100_0_0_0;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
    println!("num of spaces: {}", spaces);

    let (a, b) = r().overflowing_add(150);
    if b {
        println!("overflow u8: {}", a);
    }
}