fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to add an element to the vector
    vec.push(4);

    println!("Vector: {:?}", vec);
}
