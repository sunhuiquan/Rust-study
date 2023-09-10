fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("{}:{} {}:{} {}:{} {}:{}", c, std::mem::size_of_val(&c), z, std::mem::size_of_val(&z), g, std::mem::size_of_val(&g), heart_eyed_cat, std::mem::size_of_val(&heart_eyed_cat));


    let t = true;
    println!("{}", std::mem::size_of_val(&t));

    let u = ();
    println!("{}", std::mem::size_of_val(&u));
}

