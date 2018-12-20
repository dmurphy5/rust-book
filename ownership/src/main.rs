fn main() {
    let s = String::from("hello");

    let hello = &s[..=4];

    println!("{}", hello);
}
