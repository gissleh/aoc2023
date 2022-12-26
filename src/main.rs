use common::parse::Parser;

fn main() {
    let v = b'a'.repeat_n::<[u8; 4]>(2).parse(b"aaaaaa").unwrap();

    println!("Hello World! {:?}", v);
}