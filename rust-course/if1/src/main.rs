fn main() {
    let mut v = 0;
    for i in 1..10 {
        v = if i == 9 {
            continue
        } else {
            i
        }
    }
    println!("{}", v);
}
