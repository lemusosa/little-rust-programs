fn main() {
    let number = 21;
    // two immutable references prints two different pointer memory locations
    println!("memory location of pointer-1: {:p} and pointer-2: {:p}", &number, &&number);
}
