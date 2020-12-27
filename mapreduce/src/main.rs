use std::ops::Deref;

fn main() {
    let a = Some(Box::new(String::from("aa")));
    let a = a.as_ref();
    let b = a.map(move |x| x.deref()).unwrap();
    println!("{}",b);
}