fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The value of x, y, z is: {}, {}, {}", a, b, c);

    // -----------------------------------------------------------

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
