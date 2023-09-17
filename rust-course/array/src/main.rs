fn display_array(arr: [i32; 3]) {
    println!("{:?}", arr);
}

fn display_array2(arr: &[i32]) {
    println!("{:?}", arr);
}

fn display_array3<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32;2] = [1,2];
    display_array2(&arr);

    let arr: [i8;2] = [1,2];
    display_array3(&arr);
}
