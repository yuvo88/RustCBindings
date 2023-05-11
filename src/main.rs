#[link(name = "print_in_c", kind = "static")]
extern "C" {
    fn print_in_c();
}
fn main() {
    println!("Hello, world!");
    unsafe{ print_in_c()};
}
