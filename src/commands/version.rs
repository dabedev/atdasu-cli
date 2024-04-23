const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() {
    println!("v{}", VERSION);
}
