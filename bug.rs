fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();

    // Danger zone! Modifying the vector's internal state directly.
    unsafe {
        std::ptr::write(ptr.add(len), 4);
    }

    // This will cause a panic because the vector's length is not updated.
    println!("Vector: {:?}", vec);
}