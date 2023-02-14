fn main() {
    for _ in 0..10 { // seting up ten threads
    // main finishes before the thread finishes so it might not print
    let handle = std::thread::spawn(|| {
    println!("Hello, world!");
        });
        handle.join(); // this allows us to wait for all the threads finish
    }
    
}

