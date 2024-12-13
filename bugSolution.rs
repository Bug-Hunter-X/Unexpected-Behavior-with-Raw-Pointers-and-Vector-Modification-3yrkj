fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify the first element of the vector
    v[0] = 10;
    println!("v: {:?}", v);
} 