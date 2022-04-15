// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let mut vec = fill_vec( Vec::new());

    println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec.len(), vec);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
