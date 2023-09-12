#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // thread 'main' panicked at 'not implemented', src/main.rs:13:5
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);

    // ----------------------------------------------

    let my_name = "Pascal";
    greet(my_name.to_string());

    let r = &mut 1;
    *r = 2;
    println!("r: {}, {}", r, *r);

    let t1 = &1;
    let t2 = t1;
    println!("r: {}, {}", t1, t2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let s = [2, 3];
    let s1 = &s[0..2];
}
