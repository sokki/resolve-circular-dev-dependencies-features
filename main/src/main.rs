fn main() {
    println!("Hello, world!");
    #[cfg(feature = "testing")]
    println!("feature testing is enabled");
}
