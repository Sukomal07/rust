fn main() {
    let s = "Hello world!"; // this is string

    let mut text = String::from("hello");

    {
        let x = "Hello world!";

        println!("X = {x}");
    }

    text.push_str(" world");

    println!("text = {text}");

    println!("S = {s}");

    // println!("X = {x}"); //cannot access a value outside of the scope
}
