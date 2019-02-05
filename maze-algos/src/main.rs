fn main() {
    
    // Testing floating point equality
    let f = 10.0f32/2.0f32;
    
    if f != f.trunc() {
        println!("Not equal");
    } else {
        println!("Equal")
    }
}
