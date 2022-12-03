fn main() {
    // let config = Some(3u8);
    let config = Some(2);
    if let Some(n) = config {
        println!("config: {}", n);
    }
}
